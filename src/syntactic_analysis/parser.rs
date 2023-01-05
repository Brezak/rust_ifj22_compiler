mod ident;
mod expression;
mod control;

use crate::lexer::Span;
use crate::syntactic_analysis::ast::{Body, MainBody, Type, WithSpan};
use crate::syntactic_analysis::helpers::expected_types;
use crate::token::Token;
use crate::{error, unreachable, warn};
use chumsky::prelude::*;
use std::hash::{Hash, Hasher};

fn check_ident(span: Span, ident: String) -> Result<WithSpan<String>, Simple<Token>> {
    let mut chars = ident.chars();
    if let Some(first_char) = chars.next() {
        if !first_char.is_alphanumeric() {
            return Err(error!(
                span,
                "identifiers are meant to start with an alphanumeric symbol"
            ));
        }
    } else {
        return Err(unreachable!(span, "encountered an empty identifier")); // An empty identifier shouldn't be constructable
    }

    for c in chars {
        if !(c.is_alphanumeric() || c == '_') {
            return Err(error!(
                span,
                "identifiers are meant to be composed of alphanumeric symbols and the _ symbol"
            ));
        }
    }

    Ok(WithSpan(span, ident))
}

fn func_ident() -> impl Parser<Token, WithSpan<String>, Error = Simple<Token>> + Copy {
    filter_map(|span, token| match token {
        Token::Ident(name) => check_ident(span, name),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
    })
}

fn var_ident() -> impl Parser<Token, WithSpan<String>, Error = Simple<Token>> + Copy {
    filter_map(|span, token| match token {
        Token::Ident(name) => {
            let (start, rest) = name.split_at(1);
            if start != "$" {
                return Err(error!(span, "variable identifiers should start with $"));
            }

            check_ident(span, rest.to_string()).map(|WithSpan(span, _)| WithSpan(span, name))
        }

        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
    })
}

fn type_parser() -> impl Parser<Token, WithSpan<Type>, Error = Simple<Token>> + Copy {
    filter_map(|span: Span, token: Token| match token {
        Token::Ident(ref name) => {
            let nullable = name.starts_with('?');
            let r#type = if nullable { name.split_at(1).1 } else { &name };

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
    let function_name = func_ident();

    let var_ident = var_ident();

    todo()
}

pub fn parser() -> impl Parser<Token, MainBody, Error = Simple<Token>> {
    let function_name = func_ident();

    let var_ident = var_ident();

    let type_parser = type_parser();

    let arg = type_parser.then(var_ident);

    let args = arg
        .separated_by(just(Token::Control(',')))
        .allow_trailing()
        .delimited_by(just(Token::Control('(')), just(Token::Control(')')))
        .labelled("function args");

    let func = ();
    todo()
}
