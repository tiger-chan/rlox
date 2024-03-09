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

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

pub trait Visitor<S> {
    fn accept(&self, expr: &Expr) -> S {
        match &expr {
            Expr::Binary(bin) => self.visit_binary(bin),
            Expr::Grouping(group) => self.visit_grouping(group),
            Expr::Literal(literal) => self.visit_literal(literal),
            Expr::Unary(unary) => self.visit_unary(unary),
        }
    }

    fn visit_binary(&self, expr: &Binary) -> S;
    fn visit_grouping(&self, expr: &Grouping) -> S;
    fn visit_literal(&self, expr: &Literal) -> S;
    fn visit_unary(&self, expr: &Unary) -> S;
}

impl Expression for Expr {}
