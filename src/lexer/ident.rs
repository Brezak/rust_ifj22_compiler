use crate::Token;
use chumsky::prelude::*;

pub fn ident() -> impl Parser<char, Token, Error = Simple<char>> + Clone {
    filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_' || *c == '$')
        .chain(filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_').repeated())
        .collect()
        .map(Token::Ident)
}

pub fn keyword() -> impl Parser<char, Token, Error = Simple<char>> + Clone {
    choice((
        text::keyword("function").to(Token::Function),
        text::keyword("if").to(Token::If),
        text::keyword("else").to(Token::Else),
        text::keyword("elseif").to(Token::ElseIf),
        text::keyword("true").to(Token::Bool(true)),
        text::keyword("false").to(Token::Bool(false)),
        text::keyword("null").to(Token::Null),
        text::keyword("while").to(Token::While),
        text::keyword("return").to(Token::Return),
        text::keyword("for").to(Token::For),
        text::keyword("continue").to(Token::Continue),
        text::keyword("break").to(Token::Break),
    ))
}
