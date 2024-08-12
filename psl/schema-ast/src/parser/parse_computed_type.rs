use super::{
    helpers::{parsing_catch_all, Pair},
    Rule,
    parse_computed_type_arguments::parse_computed_type_arguments_list,
};
use crate::ast::*;
use diagnostics::{Diagnostics, FileId};

/////////////////////////
/// Parse a computed type such as $tables(A, B)
/////////////////////////

pub(crate) fn parse_computed_type(pair: Pair<'_>, diagnostics: &mut Diagnostics, file_id: FileId) -> FieldType {

    //let rule1 = pair.as_rule();

    match pair.as_rule() {
        Rule::computed_type_function_call => FieldType::Supported(FieldValue::ComputedType(parse_computed_type_function_call(pair, diagnostics, file_id))),
        _ => unreachable!("Encountered impossible computed type during parsing: {:?}", pair.tokens()),
    }
}

pub(crate) fn parse_computed_type_function_call(pair: Pair<'_>, diagnostics: &mut Diagnostics, file_id: FileId) -> ComputedTypeExpression {
    let mut name: Option<String> = None;
    let mut arguments = ComputedTypeArgumentsList::default();
    let (pair_str, span) = (pair.as_str(), pair.as_span());

    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::path => name = Some(current.as_str().to_string()),
            Rule::computed_type_arguments_list => parse_computed_type_arguments_list(current, &mut arguments, diagnostics, file_id),
            _ => parsing_catch_all(&current, "function"),
        }
    }

    match name {
        Some(name) => ComputedTypeExpression { 
            name: name,
            arguments: arguments,
            span: Span::from((file_id, span))
        },
        _ => unreachable!("Encountered impossible function during parsing: {:?}", pair_str),
    }
}