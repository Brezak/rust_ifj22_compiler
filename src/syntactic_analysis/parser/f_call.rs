use chumsky::prelude::*;

use crate::syntactic_analysis::ast::RVal;
use crate::syntactic_analysis::parser::expression::{expression, expression_to_reverse_polish};
use crate::syntactic_analysis::parser::ident::func_ident;
use crate::Token;

pub fn function_call() -> impl Parser<Token, RVal, Error = Simple<Token>> + Clone {
    let control = |c| just(Token::Control(c));

    func_ident()
        .then(
            expression()
                .map(expression_to_reverse_polish)
                .separated_by(control(','))
                .allow_trailing()
                .delimited_by(control('('), control(')')),
        )
        .map(|(ident, args)| RVal::FunctionCall { ident, args })
}
