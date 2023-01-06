use std::collections::VecDeque;
use std::str::FromStr;

use chumsky::prelude::*;

use crate::syntactic_analysis::ast::{Expression, Term};
use crate::{error, Span, Token};

pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Concat(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    More(Box<Expr>, Box<Expr>),
    LessEq(Box<Expr>, Box<Expr>),
    MoreEq(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    NEq(Box<Expr>, Box<Expr>),
    Var(String),
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
}

fn parse_num(span: Span, num: String) -> Result<Expr, Simple<Token>> {
    if num.contains(['e', 'E', '.']) {
        f64::from_str(&num)
            .map(Expr::Float)
            .map_err(|_| error!(span, format!("Failed to parse {num} as float")))
    } else {
        i64::from_str(&num)
            .map(Expr::Int)
            .map_err(|_| error!(span, format!("Failed to parse {num} as int")))
    }
}

pub fn expression() -> impl Parser<Token, Expr, Error = Simple<Token>> + Clone {
    recursive(|expr| {
        let num = filter_map(|span, token| match token {
            Token::Num(n) => parse_num(span, n),
            n => Err(Simple::expected_input_found(span, [], Some(n))),
        });

        let term = select! {
        Token::Null => Expr::Null,
        Token::Bool(b) => Expr::Bool(b),
        Token::Str(s) => Expr::String(s),
        Token::Ident(i) if i.starts_with('$') => Expr::Var(i),
        }
        .or(num);

        let cont = |c| just(Token::Control(c));

        let atom = term.or(expr.delimited_by(cont('('), cont(')')));

        let op = |c: &str| just(Token::Op(c.to_string()));

        // First in precedence table
        let product = atom
            .clone()
            .then(
                op("*")
                    .to(Expr::Mul as fn(_, _) -> _)
                    .or(op("/").to(Expr::Div as fn(_, _) -> _))
                    .then(atom)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        // Second in precedence table
        let sum = product
            .clone()
            .then(
                op("+")
                    .to(Expr::Add as fn(_, _) -> _)
                    .or(op("-").to(Expr::Sub as fn(_, _) -> _))
                    .or(op(".").to(Expr::Concat as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let cmp = sum
            .clone()
            .then(
                op("<")
                    .to(Expr::Less as fn(_, _) -> _)
                    .or(op(">").to(Expr::More as fn(_, _) -> _))
                    .or(op("<=").to(Expr::LessEq as fn(_, _) -> _))
                    .or(op(">=").to(Expr::MoreEq as fn(_, _) -> _))
                    .then(sum)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let eq = cmp
            .clone()
            .then(
                op("===")
                    .to(Expr::Eq as fn(_, _) -> _)
                    .or(op("==!").to(Expr::NEq as fn(_, _) -> _))
                    .then(cmp)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        eq
    })
}

pub fn expression_to_reverse_polish(expr: Expr) -> Vec<Expression> {
    let t = Expression::Term;

    let mut result = VecDeque::new();
    let mut stack = vec![expr];

    while !stack.is_empty() {
        let top = stack.pop().unwrap(); // Stack shouldn't be empty at this point

        match top {
            Expr::Add(a, b) => {
                stack.push(*a);
                stack.push(*b);
                result.push_front(Expression::Add);
            }
            Expr::Sub(a, b) => {
                stack.push(*a);
                stack.push(*b);
                result.push_front(Expression::Subtract);
            }
            Expr::Mul(a, b) => {
                stack.push(*a);
                stack.push(*b);
                result.push_front(Expression::Multiply);
            }
            Expr::Div(a, b) => {
                stack.push(*a);
                stack.push(*b);
                result.push_front(Expression::Divide);
            }
            Expr::Concat(a, b) => {
                stack.push(*a);
                stack.push(*b);
                result.push_front(Expression::Concat);
            }
            Expr::Less(a, b) => {
                stack.push(*a);
                stack.push(*b);
                result.push_front(Expression::Smaller);
            }
            Expr::More(a, b) => {
                stack.push(*a);
                stack.push(*b);
                result.push_front(Expression::Greater);
            }
            Expr::LessEq(a, b) => {
                stack.push(*a);
                stack.push(*b);
                result.push_front(Expression::SmallerEquals);
            }
            Expr::MoreEq(a, b) => {
                stack.push(*a);
                stack.push(*b);
                result.push_front(Expression::GreaterEquals);
            }
            Expr::Eq(a, b) => {
                stack.push(*a);
                stack.push(*b);
                result.push_front(Expression::Equals);
            }
            Expr::NEq(a, b) => {
                stack.push(*a);
                stack.push(*b);
                result.push_front(Expression::NotEquals);
            }
            Expr::Var(ident) => result.push_front(t(Term::Var(ident))),
            Expr::Int(i) => result.push_front(t(Term::Int(i))),
            Expr::Float(f) => result.push_front(t(Term::Float(f))),
            Expr::String(s) => result.push_front(t(Term::String(s))),
            Expr::Bool(b) => result.push_front(t(Term::Bool(b))),
            Expr::Null => result.push_front(t(Term::Null)),
        }
    }

    result.into()
}
