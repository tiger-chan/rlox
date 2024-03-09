use super::{Expr, Expression};

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    Plus,
    Minus,
    Mul,
    Div,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Eq => write!(f, "=="),
            Operator::Neq => write!(f, "!="),
            Operator::Lt => write!(f, "<"),
            Operator::Lte => write!(f, "<="),
            Operator::Gt => write!(f, ">"),
            Operator::Gte => write!(f, ">="),
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
        }
    }
}

pub struct Binary {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op: Operator,
}

impl Expression for Binary {}
