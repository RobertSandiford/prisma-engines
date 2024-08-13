use super::{
    helpers::{parsing_catch_all, Pair},
    parse_expression::parse_expression,
    Rule,
};
use crate::ast::{self, FieldType, FieldValue, Identifier, Union};
use diagnostics::{Diagnostics, FileId};

pub(crate) fn parse_union_type(pair: Pair<'_>, diagnostics: &mut Diagnostics, file_id: FileId) -> FieldType {
    assert!(pair.as_rule() == Rule::union_type);
    
    println!("parse_union_type");
    println!("pair: {pair:?}");
    println!();
    //let rule1 = pair.as_rule();

    //let i = pair.into_inner();

    let mut members: Vec<Identifier> = vec![];

    // dive into the pair to find Identifiers
    for inner in pair.clone().into_inner() {
        match inner.as_rule() {
            Rule::identifier => {
                println!("pushing identifier");
                let iden = Identifier::new(inner, file_id);
                println!("identifier: {iden:?}");
                println!();
                members.push(iden)
            },
            _ => parsing_catch_all(&inner, "union member")
            //_ => unreachable!("Encountered impossible computed type during parsing: {:?}", pair.tokens()),
        }
    };
    println!("return Union");
    let union = Union::new(
        pair,
        file_id,
        members
    );
    println!("union: {union:?}");
    println!();
    return FieldType::Supported(FieldValue::Union(
        union
    ))
}