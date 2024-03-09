use crate::lox::{Binary, Expr, Grouping, Literal, Unary};

use super::lox::Visitor;
pub struct AstPrinter {}

impl AstPrinter {
    fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> String {
        let mut result = format!("({name}");
        for expr in exprs.iter() {
            result = format!("{} {}", result, self.accept(expr));
        }
        result = format!("{})", result);
        result
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, expr: &Binary) -> String {
        self.parenthesize(format!("{}", expr.op).as_str(), &[&expr.left, &expr.right])
    }

    fn visit_grouping(&self, expr: &Grouping) -> String {
        self.parenthesize("group", &[&expr.expression])
    }

    fn visit_literal(&self, expr: &Literal) -> String {
        format!("{}", expr)
    }

    fn visit_unary(&self, expr: &Unary) -> String {
        self.parenthesize(format!("{}", expr.op).as_str(), &[&expr.right])
    }
}
