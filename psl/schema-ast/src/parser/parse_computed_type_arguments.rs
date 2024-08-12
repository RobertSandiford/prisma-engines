use super::{
    helpers::{parsing_catch_all, Pair},
    parse_expression::parse_expression,
    Rule,
};
use crate::ast::{self, ComputedTypeArgument, Identifier};
use diagnostics::{Diagnostics, FileId};

pub(crate) fn parse_computed_type_arguments_list(
    token: Pair<'_>,
    arguments: &mut ast::ComputedTypeArgumentsList,
    diagnostics: &mut Diagnostics,
    file_id: FileId,
) {
    println!("parse_computed_type_arguments_list");
    println!("token {token:#}");
    println!("token {token:?}");
    println!();
    debug_assert_eq!(token.as_rule(), Rule::computed_type_arguments_list);
    for current in token.into_inner() {
        let current_span = current.as_span();
        match current.as_rule() {
            Rule::computed_type_argument => {
                let inner = current.into_inner();
                let value = inner.into_iter().next();
                match value {
                    Some(argument) => {
                        match argument.as_rule() {
                            Rule::identifier => {
                                println!("Found identifier argument");
                                println!("argument: {argument:?}");
                                let name = argument.as_str();
                                println!("name: {name:?}");
                                let span = ast::Span::from((file_id, argument.as_span()));
                                println!("span: {span:?}");
                                //let str = span.as_str
                                arguments.arguments.push(ComputedTypeArgument {
                                    name: None,
                                    value: Identifier {
                                        name: String::from(name),
                                        span: span
                                    },
                                    span: span,
                                });
                            },
                            _ => parsing_catch_all(&argument, "computed typed argument value")
                        }
                    },
                    None => {
                        panic!("No inner inside computed_type_argument");
                    }
                }
            }
            Rule::trailing_comma => {
                arguments.trailing_comma = Some((file_id, current_span).into());
            },
            _ => parsing_catch_all(&current, "computed type arguments")
        }
    }
}

// fn parse_named_arg(pair: Pair<'_>, diagnostics: &mut Diagnostics, file_id: FileId) -> ast::Argument {
//     debug_assert_eq!(pair.as_rule(), Rule::named_argument);
//     let mut name: Option<ast::Identifier> = None;
//     let mut argument: Option<ast::Expression> = None;
//     let (pair_span, pair_str) = (pair.as_span(), pair.as_str());

//     for current in pair.into_inner() {
//         match current.as_rule() {
//             Rule::identifier => name = Some(ast::Identifier::new(current, file_id)),
//             Rule::expression => argument = Some(parse_expression(current, diagnostics, file_id)),
//             _ => parsing_catch_all(&current, "attribute argument"),
//         }
//     }

//     match (name, argument) {
//         (Some(name), Some(value)) => ast::Argument {
//             name: Some(name),
//             value,
//             span: ast::Span::from((file_id, pair_span)),
//         },
//         _ => panic!("Encountered impossible attribute arg during parsing: {pair_str:?}"),
//     }
// }
