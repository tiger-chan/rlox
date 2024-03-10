mod binary;
mod grouping;
mod literal;
mod unary;

pub use self::{
    binary::Binary,
    binary::Operator as BinaryOp,
    grouping::Grouping,
    literal::{Literal, Value},
    unary::{Operator as UnaryOp, Unary},
};

pub trait Expression {}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

#[derive(Debug, Clone)]
pub struct ExprTree(Vec<Expr>);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExprId(u32);

impl Default for ExprTree {
    fn default() -> Self {
        Self(Vec::with_capacity(50))
    }
}

impl ExprTree {
    pub fn with_capacity(capacity: u32) -> Self {
        Self(Vec::with_capacity(capacity as usize))
    }

    pub fn get(&self, id: ExprId) -> Option<&Expr> {
        self.0.get(id.0 as usize)
    }

    pub fn push<T: Into<Expr>>(&mut self, expr: T) -> ExprId {
        let i = self.0.len();
        self.0.push(expr.into());
        ExprId(
            i.try_into()
                .expect("Too many allocated nodes (cannot exceed u32::MAX)"),
        )
    }
}

pub trait Visitor<S> {
    fn accept(&self, tree: &ExprTree, expr: ExprId) -> Option<S> {
        match tree.get(expr) {
            Some(Expr::Binary(bin)) => Some(self.visit_binary(tree, bin)),
            Some(Expr::Grouping(group)) => Some(self.visit_grouping(tree, group)),
            Some(Expr::Literal(literal)) => Some(self.visit_literal(tree, literal)),
            Some(Expr::Unary(unary)) => Some(self.visit_unary(tree, unary)),
            None => None,
        }
    }

    fn visit_binary(&self, tree: &ExprTree, expr: &Binary) -> S;
    fn visit_grouping(&self, tree: &ExprTree, expr: &Grouping) -> S;
    fn visit_literal(&self, tree: &ExprTree, expr: &Literal) -> S;
    fn visit_unary(&self, tree: &ExprTree, expr: &Unary) -> S;
}

impl Expression for Expr {}
