use crate::syntactic_analysis::ast::{Expression, Term, WithSpan};
use crate::token::Token;
use crate::token::Token::Type;

pub fn expected_types() -> [Option<Token>; 4] {
    use crate::syntactic_analysis::ast::Type::*;

    [
        Some(Type(Int { nullable: false })),
        Some(Type(Float { nullable: false })),
        Some(Type(String { nullable: false })),
        Some(Type(Void)),
    ]
}
