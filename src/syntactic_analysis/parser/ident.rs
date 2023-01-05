use crate::lexer::Span;
use crate::syntactic_analysis::ast::{Body, Expression, MainBody, RVal, Term, Type, WithSpan};
use crate::{error, unreachable, warn, Token};
use chumsky::prelude::*;

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

pub fn func_ident() -> impl Parser<Token, WithSpan<String>, Error = Simple<Token>> + Copy {
    filter_map(|span, token| match token {
        Token::Ident(name) => check_ident(span, name),
        _ => Err(Simple::expected_input_found(span, Vec::new(), Some(token))),
    })
}

pub fn var_ident() -> impl Parser<Token, WithSpan<String>, Error = Simple<Token>> + Copy {
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

pub fn rval() -> impl Parser<Token, WithSpan<RVal>, Error = Simple<Token>> + Copy {
    var_ident()
        .map(|WithSpan(span, ident)| {
            WithSpan(
                span.clone(),
                RVal::Expr {
                    expr: vec![WithSpan(span, Expression::Term(Term::Var(ident)))],
                },
            )
        })
        .or(func_ident().map(|WithSpan(span, ident)| WithSpan(span, RVal::FunctionCall { ident })))
}
