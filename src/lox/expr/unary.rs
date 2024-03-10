use super::{Expr, ExprId, Expression};

#[allow(unused)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Unary {
    pub op: Operator,
    pub right: ExprId,
}

impl Unary {
    pub fn new(op: Operator, rhs: ExprId) -> Self {
        Self { op, right: rhs }
    }
}

impl From<Unary> for Expr {
    fn from(value: Unary) -> Self {
        Self::Unary(value)
    }
}

impl Expression for Unary {}
