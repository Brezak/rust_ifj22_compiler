use crate::token::Token;
use crate::{error, unreachable, warn};
use chumsky::prelude::*;
use std::ops::Range;

pub type Span = Range<usize>;

fn number() -> impl Parser<char, Token, Error = Simple<char>> {
    let exp = just('e')
        .or(just('E'))
        .chain(just('+').or(just('-')).or_not())
        .chain::<char, _, _>(text::digits(10));

    let frac = just('.').chain(text::digits(10));

    text::int(10)
        .chain::<char, _, _>(frac.or_not().flatten())
        .chain::<char, _, _>(exp.or_not().flatten())
        .collect::<String>()
        .labelled("number")
        .map(Token::Num)
}

pub fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    let num = number();

    let operator = one_of("+-*/.=!<>")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(Token::Op);

    let escape = just('\\').ignore_then(
        just('\\')
            .or(just('"').to('\"'))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t'))
            .or(just('v').to('\x0B')) // vertical tab
            .or(just('e').to('\x1B')) // escape
            .or(just('f').to('\x0C'))
            .or(
                filter(|c: &char| c.is_ascii_digit() && *c != '9' && *c != '8')
                    .repeated()
                    .exactly(3)
                    .collect::<String>()
                    .validate(|digits, span: Span, emit| {
                        if let Ok(parsed_octal) = u32::from_str_radix(&digits, 8) {
                            if parsed_octal > 0xFF {
                                emit(error!(span, "octal can't be more than 377"));
                                return '\u{FFFD}'; // unicode replacement character
                            }

                            char::from_u32(parsed_octal).unwrap_or_else(|| {
                                emit(error!(span, "invalid ascii character"));
                                '\u{FFFD}' // unicode replacement character
                            })
                        } else {
                            emit(unreachable!(span));
                            '\u{FFFD}' // unicode replacement character
                        }
                    }),
            ) // form feed
            .or(just('x').ignore_then(
                filter(|c: &char| c.is_ascii_hexdigit())
                    .repeated()
                    .exactly(2)
                    .collect::<String>()
                    .validate(|digits, span: Span, emit| {
                        if let Ok(parsed_hex) = u32::from_str_radix(&digits, 16) {
                            char::from_u32(parsed_hex).unwrap_or_else(|| {
                                emit(error!(span, "invalid ascii character")); // Should be unreachable
                                '\u{FFFD}' // unicode replacement character
                            })
                        } else {
                            emit(unreachable!(span));
                            '\u{FFFD}' // unicode replacement character
                        }
                    }),
            ))
            .or(just('u').ignore_then(
                filter(|c: &char| c.is_digit(16))
                    .repeated()
                    .exactly(4)
                    .collect::<String>()
                    .validate(|digits, span, emit| {
                        char::from_u32(u32::from_str_radix(&digits, 16).unwrap()).unwrap_or_else(
                            || {
                                emit(Simple::custom(span, "invalid unicode character"));
                                '\u{FFFD}' // unicode replacement character
                            },
                        )
                    }),
            )),
    );

    // A parser for strings
    let string = just('"')
        .ignore_then(
            filter(|c| *c != '\n' && *c != '\"' && *c != '\\')
                .or(escape)
                .repeated(),
        )
        .then_ignore(just('"'))
        .collect::<String>()
        .labelled("string")
        .map(Token::Str);

    let ctrl = one_of("()[]{};,").map(Token::Control);

    let php_ident = filter(|c: &char| c.is_alphabetic() || *c == '_' || *c == '$')
        .map(Some)
        .chain::<char, Vec<_>, _>(
            filter(|c: &char| c.is_alphabetic() || *c == '_' || *c == '$').repeated(),
        )
        .collect();

    let ident = choice((
        text::keyword("function").to(Token::Function),
        text::keyword("if").to(Token::If),
        text::keyword("else").to(Token::Else),
        text::keyword("true").to(Token::Bool(true)),
        text::keyword("false").to(Token::Bool(false)),
        text::keyword("null").to(Token::Null),
        text::keyword("while").to(Token::While),
        text::keyword("return").to(Token::Return),
        php_ident.map(Token::Ident),
    ));

    let token = num
        .or(string)
        .or(operator)
        .or(ctrl)
        .or(ident)
        .recover_with(skip_then_retry_until([]));

    let comment = just("//")
        .then(take_until(just("\n")))
        .or(just("/*").then(take_until(just("*/"))))
        .padded();

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
}
