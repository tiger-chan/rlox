use super::{Expr, Expression};

pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Expression for Grouping {}
