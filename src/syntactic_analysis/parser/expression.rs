use chumsky::prelude::*;

use crate::syntactic_analysis::ast::{Expression, WithSpan};
use crate::Token;

pub fn expression(
) -> impl Parser<Token, WithSpan<Vec<WithSpan<Expression>>>, Error = Simple<Token>> + Copy {
    todo()
}
