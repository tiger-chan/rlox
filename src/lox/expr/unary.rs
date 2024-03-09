use super::{Expr, Expression};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Minus,
    Bang,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Minus => write!(f, "-"),
            Operator::Bang => write!(f, "!"),
        }
    }
}

pub struct Unary {
    pub op: Operator,
    pub right: Box<Expr>,
}

impl Expression for Unary {}
