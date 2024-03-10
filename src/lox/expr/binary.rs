use super::{Expr, ExprId, Expression};

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Binary {
    pub left: ExprId,
    pub right: ExprId,
    pub op: Operator,
}

impl Binary {
    pub fn new(lhs: ExprId, op: Operator, rhs: ExprId) -> Self {
        Self {
            left: lhs,
            op,
            right: rhs,
        }
    }
}

impl From<Binary> for Expr {
    fn from(value: Binary) -> Self {
        Self::Binary(value)
    }
}

impl Expression for Binary {}
