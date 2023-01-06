use chumsky::prelude::*;

use crate::syntactic_analysis::ast::{Expression, Term, WithSpan};
use crate::syntactic_analysis::parser::ident::var_ident;
use crate::Token;

pub fn expression(
) -> impl Parser<Token, WithSpan<Vec<WithSpan<Expression>>>, Error = Simple<Token>> + Copy {
    // recursive(|expr| {
    //     let op
    //
    // });

    todo()
}
