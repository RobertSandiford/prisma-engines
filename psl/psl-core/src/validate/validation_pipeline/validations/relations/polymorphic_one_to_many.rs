use super::*;
use crate::{diagnostics::DatamodelError, validate::validation_pipeline::context::Context};
use parser_database::ast::WithSpan;

/// A relation should have the explicit and back-relation side defined.
pub(crate) fn both_sides_are_defined(relation: InlineRelationWalker<'_>, ctx: &mut Context<'_>) {

}

/// The singular side must define `fields` and `references` attributes.
pub(crate) fn fields_and_references_are_defined(relation: InlineRelationWalker<'_>, ctx: &mut Context<'_>) {

}

/// The referential actions, if defined, must be on the singular side only.
pub(crate) fn referential_actions(relation: InlineRelationWalker<'_>, ctx: &mut Context<'_>) {

}