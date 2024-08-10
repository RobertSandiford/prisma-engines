use super::{
    helpers::{parsing_catch_all, Pair},
    Rule,
    parse_arguments::parse_arguments_list,
};
use crate::{ast::*, parser::parse_expression::parse_expression};
use diagnostics::{DatamodelError, Diagnostics, FileId};

pub fn parse_field_type(
    pair: Pair<'_>,
    diagnostics: &mut Diagnostics,
    file_id: FileId,
) -> Result<(FieldArity, FieldType), DatamodelError> {
    assert!(pair.as_rule() == Rule::field_type);
    let current = pair.into_inner().next().unwrap();
    match current.as_rule() {
        Rule::optional_type => Ok((
            FieldArity::Optional,
            parse_base_type(current.into_inner().next().unwrap(), diagnostics, file_id),
        )),
        Rule::base_type => Ok((FieldArity::Required, parse_base_type(current, diagnostics, file_id))),
        Rule::list_type => Ok((
            FieldArity::List,
            parse_base_type(current.into_inner().next().unwrap(), diagnostics, file_id),
        )),
        Rule::legacy_required_type => Err(DatamodelError::new_legacy_parser_error(
            "Fields are required by default, `!` is no longer required.",
            (file_id, current.as_span()).into(),
        )),
        Rule::legacy_list_type => Err(DatamodelError::new_legacy_parser_error(
            "To specify a list, please use `Type[]` instead of `[Type]`.",
            (file_id, current.as_span()).into(),
        )),
        Rule::unsupported_optional_list_type => Err(DatamodelError::new_legacy_parser_error(
            "Optional lists are not supported. Use either `Type[]` or `Type?`.",
            (file_id, current.as_span()).into(),
        )),
        Rule::computed_type => Ok((
            FieldArity::Required,
            parse_computed_type(current.into_inner().next().unwrap(), diagnostics, file_id),
        )),
        _ => unreachable!("Encountered impossible field during parsing: {:?}", current.tokens()),
    }
}

fn parse_base_type(pair: Pair<'_>, diagnostics: &mut Diagnostics, file_id: FileId) -> FieldType {
    let current = pair.into_inner().next().unwrap();
    match current.as_rule() {
        Rule::identifier => FieldType::Supported(FieldValue::Identifier(Identifier {
            name: current.as_str().to_string(),
            span: Span::from((file_id, current.as_span())),
        })),
        Rule::unsupported_type => match parse_expression(current, diagnostics, file_id) {
            Expression::StringValue(lit, span) => FieldType::Unsupported(lit, span),
            _ => unreachable!("Encountered impossible type during parsing"),
        },
        _ => unreachable!("Encountered impossible type during parsing: {:?}", current.tokens()),
    }
}

fn parse_computed_type(pair: Pair<'_>, diagnostics: &mut Diagnostics, file_id: FileId) -> FieldType {
    println!("pair: {pair:?}");
    println!("");
    let rule1 = pair.as_rule();
    println!("rule1: {rule1:?}");
    println!("diagnostics: {diagnostics:?}");
    println!("");
    println!("file_id: {file_id:?}");
    println!("");

    //let current = pair.into_inner().next().unwrap();
    //println!("current: {current:?}");
    //println!("");
    //let rule2 = current.as_rule();
    //println!("rule2: {rule2:?}");

    match pair.as_rule() {
        Rule::function_call => FieldType::Supported(FieldValue::ComputedType(parse_function_call(pair, diagnostics, file_id))),
        // FieldType::Supported(Identifier {
        //     name: current.as_str().to_string(),
        //     span: Span::from((file_id, current.as_span())),
        // }),
        _ => unreachable!("Encountered impossible computed type during parsing: {:?}", pair.tokens()),
    }
    // FieldType::Supported(Identifier {
    //     name: String::from("a-name"),
    //     span: Span::from((file_id, current.as_span())),
    // })
}

fn parse_function_call(pair: Pair<'_>, diagnostics: &mut Diagnostics, file_id: FileId) -> ComputedTypeExpression {
    let mut name: Option<String> = None;
    let mut arguments = ArgumentsList::default();
    let (pair_str, span) = (pair.as_str(), pair.as_span());

    for current in pair.into_inner() {
        match current.as_rule() {
            Rule::path => name = Some(current.as_str().to_string()),
            Rule::arguments_list => parse_arguments_list(current, &mut arguments, diagnostics, file_id),
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