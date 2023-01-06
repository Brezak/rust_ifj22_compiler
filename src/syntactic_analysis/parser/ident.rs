use chumsky::prelude::*;

use crate::syntactic_analysis::ast::RVal;
use crate::syntactic_analysis::parser::expression::{expression, expression_to_reverse_polish};
use crate::syntactic_analysis::parser::f_call::function_call;
use crate::{error, unreachable, Span, Token};

fn check_ident(span: Span, ident: String) -> Result<String, Simple<Token>> {
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

    Ok(ident)
}

pub fn func_ident() -> impl Parser<Token, String, Error = Simple<Token>> + Copy {
    filter_map(|span, token| match token {
        Token::Ident(name) => check_ident(span, name),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
    })
}

pub fn var_ident() -> impl Parser<Token, String, Error = Simple<Token>> + Copy {
    filter_map(|span, token| match token {
        Token::Ident(name) => {
            let (start, rest) = name.split_at(1);
            if start != "$" {
                return Err(error!(span, "variable identifiers should start with $"));
            }

            check_ident(span, rest.to_string())
        }

        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
    })
}

pub fn rval() -> impl Parser<Token, RVal, Error = Simple<Token>> + Clone {
    expression()
        .map(|expr| RVal::Expr {
            expr: expression_to_reverse_polish(expr),
        })
        .or(function_call())
}
