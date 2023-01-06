use chumsky::prelude::*;

use crate::syntactic_analysis::ast::{If, While};
use crate::syntactic_analysis::parser::body::body;
use crate::syntactic_analysis::parser::ident::rval;
use crate::Token;

pub fn if_block() -> impl Parser<Token, If, Error = Simple<Token>> + Clone {
    let control = |c| just(Token::Control(c));

    just(Token::If)
        .ignore_then(rval().delimited_by(control('('), control(')')))
        .then(body().delimited_by(control('{'), control('}')))
        .then_ignore(just(Token::Else))
        .then(body().delimited_by(control('{'), control('}')))
        .map(|((expr, if_body), else_body)| If {
            expr,
            if_body,
            else_body,
        })
}

pub fn while_block() -> impl Parser<Token, While, Error = Simple<Token>> + Clone {
    let control = |c| just(Token::Control(c));

    just(Token::While)
        .ignore_then(rval().delimited_by(control('('), control(')')))
        .then(body().delimited_by(control('{'), control('}')))
        .map(|(expr, body)| While { expr, body })
}
