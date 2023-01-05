use std::fmt::{Debug, Display, Formatter};

use crate::syntactic_analysis::ast::Type;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Token {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Op(String),
    Control(char),
    Ident(String),
    Function,
    If,
    Else,
    Return,
    While,
    Type(Type),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Null => write!(f, "null"),
            Token::Bool(n) => write!(f, "{n}"),
            Token::Num(n) => write!(f, "{n}"),
            Token::Str(n) => write!(f, "{n}"),
            Token::Op(n) => write!(f, "{n}"),
            Token::Control(n) => write!(f, "{n}"),
            Token::Ident(n) => write!(f, "{n}"),
            Token::Function => write!(f, "function"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
            Token::While => write!(f, "while"),
            Token::Type(name) => Display::fmt(name, f),
        }
    }
}
