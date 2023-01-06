mod body;
mod control;
mod expression;
mod f_call;
mod ident;

use chumsky::prelude::*;

use crate::syntactic_analysis::ast::{Body, MainBody, Type};
use crate::syntactic_analysis::helpers::expected_types;
use crate::syntactic_analysis::parser::ident::{func_ident, var_ident};
use crate::token::Token;
use crate::{error, Span, WithSpan};

fn type_parser() -> impl Parser<Token, WithSpan<Type>, Error = Simple<Token>> + Copy {
    filter_map(|span: Span, token: Token| match token {
        Token::Ident(ref name) => {
            let nullable = name.starts_with('?');
            let r#type = if nullable { name.split_at(1).1 } else { name };

            match r#type {
                "int" => Ok(WithSpan(span, Type::Int { nullable })),
                "float" => Ok(WithSpan(span, Type::Float { nullable })),
                "string" => Ok(WithSpan(span, Type::String { nullable })),
                "void" if !nullable => Ok(WithSpan(span, Type::Void)),
                "void" if nullable => Err(error!(span, "a void return value is non nullable")),
                _ => Err(Simple::expected_input_found(
                    span,
                    expected_types(),
                    Some(token),
                )),
            }
        }
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
    })
}

pub fn parse_body() -> impl Parser<Token, WithSpan<Vec<WithSpan<Body>>>, Error = Simple<Token>> {
    let _function_name = func_ident();

    let _var_ident = var_ident();

    todo()
}

pub fn parser() -> impl Parser<Token, MainBody, Error = Simple<Token>> {
    let _function_name = func_ident();

    let var_ident = var_ident();

    let type_parser = type_parser();

    let arg = type_parser.then(var_ident);

    let _args = arg
        .separated_by(just(Token::Control(',')))
        .allow_trailing()
        .delimited_by(just(Token::Control('(')), just(Token::Control(')')))
        .labelled("function args");

    ();
    todo()
}
