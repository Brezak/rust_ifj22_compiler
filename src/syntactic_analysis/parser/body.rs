use chumsky::prelude::*;

use crate::syntactic_analysis::ast::Body;
use crate::Token;

pub fn body() -> impl Parser<Token, Vec<Body>, Error = Simple<Token>> + Copy {
    todo()
}
