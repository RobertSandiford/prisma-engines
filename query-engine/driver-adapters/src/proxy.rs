use crate::send_future::UnsafeFuture;
pub use crate::types::{ColumnType, JSResultSet, Query, TransactionOptions};
use crate::{from_js_value, get_named_property, has_named_property, to_rust_str, JsObject, JsResult, JsString};

use crate::{AsyncJsFunction, JsTransaction};
use futures::Future;
use metrics::increment_gauge;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Proxy is a struct wrapping a javascript object that exhibits basic primitives for
/// querying and executing SQL (i.e. a client connector). The Proxy uses Napi/Wasm's JsFunction
/// to invoke the code within the node runtime that implements the client connector.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub(crate) struct CommonProxy {
    /// Execute a query given as SQL, interpolating the given parameters.
    query_raw: AsyncJsFunction<Query, JSResultSet>,

    /// Execute a query given as SQL, interpolating the given parameters and
    /// returning the number of affected rows.
    execute_raw: AsyncJsFunction<Query, u32>,

    /// Return the provider for this driver.
    pub(crate) provider: String,
}

/// This is a JS proxy for accessing the methods specific to top level
/// JS driver objects
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub(crate) struct DriverProxy {
    start_transaction: AsyncJsFunction<(), JsTransaction>,
}

/// This a JS proxy for accessing the methods, specific
/// to JS transaction objects
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub(crate) struct TransactionProxy {
    /// transaction options
    options: TransactionOptions,

    /// commit transaction
    commit: AsyncJsFunction<(), ()>,

    /// rollback transaction
    rollback: AsyncJsFunction<(), ()>,

    /// whether the transaction has already been committed or rolled back
    closed: AtomicBool,
}

impl CommonProxy {
    pub fn new(object: &JsObject) -> JsResult<Self> {
        // Background info:
        // - the provider was previously called "flavour", so we provide a temporary fallback for third-party providers
        //   to give them time to adapt
        // - reading a named property that does not exist yields a panic, despite the `Result<_, _>` return type
        let provider: JsString = if has_named_property(object, "provider")? {
            get_named_property(object, "provider")?
        } else {
            get_named_property(object, "flavour")?
        };

        Ok(Self {
            query_raw: get_named_property(object, "queryRaw")?,
            execute_raw: get_named_property(object, "executeRaw")?,
            provider: to_rust_str(provider)?,
        })
    }

    pub async fn query_raw(&self, params: Query) -> quaint::Result<JSResultSet> {
        self.query_raw.call(params).await
    }

    pub async fn execute_raw(&self, params: Query) -> quaint::Result<u32> {
        self.execute_raw.call(params).await
    }
}

impl DriverProxy {
    pub fn new(object: &JsObject) -> JsResult<Self> {
        Ok(Self {
            start_transaction: get_named_property(object, "startTransaction")?,
        })
    }

    async fn start_transaction_inner(&self) -> quaint::Result<Box<JsTransaction>> {
        let tx = self.start_transaction.call(()).await?;

        // Decrement for this gauge is done in JsTransaction::commit/JsTransaction::rollback
        // Previously, it was done in JsTransaction::new, similar to the native Transaction.
        // However, correct Dispatcher is lost there and increment does not register, so we moved
        // it here instead.
        increment_gauge!("prisma_client_queries_active", 1.0);
        Ok(Box::new(tx))
    }

    pub fn start_transaction(&self) -> UnsafeFuture<impl Future<Output = quaint::Result<Box<JsTransaction>>> + '_> {
        UnsafeFuture(self.start_transaction_inner())
    }
}

impl TransactionProxy {
    pub fn new(js_transaction: &JsObject) -> JsResult<Self> {
        let commit = get_named_property(js_transaction, "commit")?;
        let rollback = get_named_property(js_transaction, "rollback")?;
        let options = get_named_property(js_transaction, "options")?;
        let options = from_js_value::<TransactionOptions>(options);

        Ok(Self {
            commit,
            rollback,
            options,
            closed: AtomicBool::new(false),
        })
    }

    pub fn options(&self) -> &TransactionOptions {
        &self.options
    }

    /// Commits the transaction via the driver adapter.
    ///
    /// ## Cancellation safety
    ///
    /// The future is cancellation-safe as long as the underlying Node-API call
    /// is cancellation-safe and no new await points are introduced between storing true in
    /// [`TransactionProxy::closed`] and calling the underlying JS function.
    ///
    /// - If `commit` is called but never polled or awaited, it's a no-op, the transaction won't be
    ///   committed and [`TransactionProxy::closed`] will not be changed.
    ///
    /// - If it is polled at least once, `true` will be stored in [`TransactionProxy::closed`] and
    ///   the underlying FFI call will be delivered to JavaScript side in lockstep, so the destructor
    ///   will not attempt rolling the transaction back even if the `commit` future was dropped while
    ///   waiting on the JavaScript call to complete and deliver response.
    pub fn commit(&self) -> UnsafeFuture<impl Future<Output = quaint::Result<()>> + '_> {
        self.closed.store(true, Ordering::Relaxed);
        UnsafeFuture(self.commit.call(()))
    }

    /// Rolls back the transaction via the driver adapter.
    ///
    /// ## Cancellation safety
    ///
    /// The future is cancellation-safe as long as the underlying Node-API call
    /// is cancellation-safe and no new await points are introduced between storing true in
    /// [`TransactionProxy::closed`] and calling the underlying JS function.
    ///
    /// - If `rollback` is called but never polled or awaited, it's a no-op, the transaction won't be
    ///   rolled back yet and [`TransactionProxy::closed`] will not be changed.
    ///
    /// - If it is polled at least once, `true` will be stored in [`TransactionProxy::closed`] and
    ///   the underlying FFI call will be delivered to JavaScript side in lockstep, so the destructor
    ///   will not attempt rolling back again even if the `rollback` future was dropped while waiting
    ///   on the JavaScript call to complete and deliver response.
    pub fn rollback(&self) -> UnsafeFuture<impl Future<Output = quaint::Result<()>> + '_> {
        self.closed.store(true, Ordering::Relaxed);
        UnsafeFuture(self.rollback.call(()))
    }
}

impl Drop for TransactionProxy {
    fn drop(&mut self) {
        if self.closed.swap(true, Ordering::Relaxed) {
            return;
        }

        self.rollback.call_non_blocking(());
    }
}

macro_rules! impl_send_sync_on_wasm {
    ($struct:ident) => {
        #[cfg(target_arch = "wasm32")]
        unsafe impl Send for $struct {}
        #[cfg(target_arch = "wasm32")]
        unsafe impl Sync for $struct {}
    };
}

// Assume the proxy object will not be sent to service workers, we can unsafe impl Send + Sync.
impl_send_sync_on_wasm!(TransactionProxy);
impl_send_sync_on_wasm!(DriverProxy);
impl_send_sync_on_wasm!(CommonProxy);
impl_send_sync_on_wasm!(JsTransaction);
