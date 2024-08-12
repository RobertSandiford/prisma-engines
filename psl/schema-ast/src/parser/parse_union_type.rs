use super::{
    helpers::{parsing_catch_all, Pair},
    parse_expression::parse_expression,
    Rule,
};
use crate::ast::{self, FieldType, FieldValue, Identifier, Union};
use diagnostics::{Diagnostics, FileId};

pub(crate) fn parse_union_type(pair: Pair<'_>, diagnostics: &mut Diagnostics, file_id: FileId) -> FieldType {

    //let rule1 = pair.as_rule();

    match pair.as_rule() {
        Rule::computed_type_function_call => {
            let mut members: Vec<Identifier> = vec![];
            // dive into the pair to find Identifiers
            for mem in pair.clone().into_inner().into_iter() {
                match mem.as_rule() {
                    Rule::identifier => members.push(Identifier::new(mem, file_id)),
                    _ =>  parsing_catch_all(&mem, "union member")
                }
            }
            return FieldType::Supported(FieldValue::Union(
                Union::new(
                    pair,
                    file_id,
                    members
                )
            ))
        },
        _ => unreachable!("Encountered impossible computed type during parsing: {:?}", pair.tokens()),
    }
}