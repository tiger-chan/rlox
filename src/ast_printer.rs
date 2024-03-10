use crate::lox::{Binary, ExprId, ExprTree, Grouping, Literal, Unary};

use super::lox::Visitor;
pub struct AstPrinter {}

impl AstPrinter {
    fn parenthesize(&self, tree: &ExprTree, name: &str, exprs: &[ExprId]) -> String {
        let mut result = format!("({name}");
        for expr in exprs.iter() {
            if let Some(x) = self.accept(tree, *expr) {
                result = format!("{} {}", result, x);
            }
        }
        result = format!("{})", result);
        result
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, tree: &ExprTree, expr: &Binary) -> String {
        self.parenthesize(
            tree,
            format!("{}", expr.op).as_str(),
            &[expr.left, expr.right],
        )
    }

    fn visit_grouping(&self, tree: &ExprTree, expr: &Grouping) -> String {
        self.parenthesize(tree, "group", &[expr.expression])
    }

    fn visit_literal(&self, _: &ExprTree, expr: &Literal) -> String {
        format!("{}", expr)
    }

    fn visit_unary(&self, tree: &ExprTree, expr: &Unary) -> String {
        self.parenthesize(tree, format!("{}", expr.op).as_str(), &[expr.right])
    }
}
