use super::{Identifier, Span, WithSpan};
use diagnostics::FileId;

/// An identifier.
#[derive(Debug, Clone, PartialEq)]
pub struct Union {
    /// The identifier contents.
    pub name: String,
    // The Identifiers within the union
    pub members: Vec<Identifier>,
    /// The span of the AST node.
    pub span: Span,
}

impl Union {
    pub(crate) fn new<T: pest::RuleType>(pair: pest::iterators::Pair<'_, T>, file_id: FileId, members: Vec<Identifier>) -> Self {
        Union {
            name: pair.as_str().to_owned(),
            members,
            span: (file_id, pair.as_span()).into(),
        }
    }
}

impl WithSpan for Union {
    fn span(&self) -> Span {
        self.span
    }
}
