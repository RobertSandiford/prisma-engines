use super::*;
use crate::{diagnostics::DatamodelError, validate::validation_pipeline::context::Context};
use parser_database::ast::WithSpan;

/// A relation should have the explicit and back-relation side defined.
pub(crate) fn both_sides_are_defined(relation: InlineRelationWalker<'_>, ctx: &mut Context<'_>) {
    
}

/// The forward side must define `fields` and `references` in the `@relation` attribute.
pub(crate) fn fields_and_references_are_defined(relation: InlineRelationWalker<'_>, ctx: &mut Context<'_>) {

}

/// `fields` and `references` should only be defined in the forward side of the relation.
pub(crate) fn fields_and_references_defined_on_one_side_only(
    relation: InlineRelationWalker<'_>,
    ctx: &mut Context<'_>,
) {
    
}