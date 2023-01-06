use std::fmt::{Display, Formatter};
use std::hash::Hash;

use crate::Span;

#[derive(Debug)]
pub enum MainBody {
    FuncDef {
        func_ident: String,
        args: Vec<Param>,
        return_ty: Type,
        body: Vec<Body>,
    },

    If(If),

    While(While),

    Exp(RVal),

    VarAssignment {
        ident: String,
        expression: RVal,
    },

    Return,
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    If {},

    While {
        span: Span,
        expr: RVal,
        body: Vec<Body>,
    },

    Exp(RVal),

    VarAssignment {
        ident: String,
        expression: RVal,
    },

    Return(RVal),
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct If {
    pub(crate) expr: RVal,
    pub(crate) if_body: Vec<Body>,
    pub(crate) else_body: Vec<Body>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct While {
    pub expr: RVal,
    pub body: Vec<Body>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RVal {
    FunctionCall {
        ident: String,
        args: Vec<Vec<Expression>>,
    },

    Expr {
        expr: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Add,
    Subtract,
    Multiply,
    Divide,
    Concat,
    Smaller,
    Greater,
    Equals,
    NotEquals,
    SmallerEquals,
    GreaterEquals,
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
    Bool(bool),
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
    pub ident: String,
    pub r#type: Type,
}
