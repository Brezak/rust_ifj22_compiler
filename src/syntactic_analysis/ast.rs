use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

use crate::lexer::Span;

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
pub struct WithSpan<T>(pub Span, pub T);

// For ergonomics
impl<T> Deref for WithSpan<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<T> DerefMut for WithSpan<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

#[derive(Debug)]
pub enum MainBody {
    FuncDef {
        span: Span,
        func_ident: WithSpan<String>,
        args: WithSpan<Vec<WithSpan<Param>>>,
        return_ty: WithSpan<Type>,
        body: Vec<WithSpan<Body>>,
    },

    If(If),

    While(While),

    Exp(RVal),

    VarAssignment {
        ident: WithSpan<String>,
        expression: WithSpan<Expression>,
    },

    Return,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    If {},

    While {
        span: Span,
        expr: WithSpan<RVal>,
        body: WithSpan<Vec<WithSpan<Body>>>,
    },

    Exp(WithSpan<RVal>),

    VarAssignment {
        ident: WithSpan<String>,
        expression: WithSpan<Expression>,
    },

    Return(WithSpan<RVal>),
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    span: Span,
    expr: WithSpan<RVal>,
    true_body: WithSpan<Vec<WithSpan<Body>>>,
    false_body: WithSpan<Vec<WithSpan<Body>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    span: Span,
    expr: WithSpan<RVal>,
    body: WithSpan<Vec<WithSpan<Body>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RVal {
    FunctionCall { ident: String },

    Expr { expr: Vec<WithSpan<Expression>> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Add,
    Subtract,
    Multiply,
    Divide,
    Concat,
    Less,
    More,
    Equals,
    NotEquals,
    LessEquals,
    MoreEquals,
    Not,
    Term(Term),
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Operation {}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Var(String),
    Int(i64), // The original language supported only i32 but why should we limit ourselves to 32 bits
    Float(f64), // Especially when it used a 64 bit floating point type
    String(String),
    Null,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Type {
    Int { nullable: bool },
    Float { nullable: bool },
    String { nullable: bool },
    Void,
}

impl Type {
    pub fn is_nullable(&self) -> bool {
        match self {
            Type::Int { nullable } => *nullable,
            Type::Float { nullable } => *nullable,
            Type::String { nullable } => *nullable,
            Type::Void => false,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Type::Int { .. } => "int",
            Type::Float { .. } => "float",
            Type::String { .. } => "string",
            Type::Void => "void",
        };

        if self.is_nullable() {
            write!(f, "?{name}")
        } else {
            write!(f, "{name}")
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Param {
    pub ident: WithSpan<String>,
    pub r#type: WithSpan<Type>,
}
