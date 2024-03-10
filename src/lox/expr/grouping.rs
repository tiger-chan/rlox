use super::{Expr, ExprId, Expression};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grouping {
    pub expression: ExprId,
}

impl Grouping {
    pub fn new(expression: ExprId) -> Self {
        Self { expression }
    }
}

impl From<Grouping> for Expr {
    fn from(value: Grouping) -> Self {
        Self::Grouping(value)
    }
}

impl Expression for Grouping {}
