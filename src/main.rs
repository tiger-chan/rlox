mod ast_printer;
mod lox;
mod string_reader;

use ast_printer::AstPrinter;
use lox::{Binary, ExprTree, Grouping, Literal, Unary, Visitor};
use string_reader::StringReader;

use std::{env, error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut tree = ExprTree::with_capacity(20);
    let binary = {
        let unary = {
            let lit = tree.push(Literal::new(123.0));
            tree.push(Unary::new(lox::UnaryOp::Minus, lit))
        };

        let grouping = {
            let lit = tree.push(Literal::new(45.67));
            tree.push(Grouping::new(lit))
        };

        tree.push(Binary::new(unary, lox::BinaryOp::Mul, grouping))
    };

    let ast_printer = AstPrinter {};
    if let Some(v) = ast_printer.accept(&tree, binary) {
        println!("Ast Printer: {}", v);
    } else {
        println!("Ast failed to find node");
    }

    match args.len() {
        0 => {}
        1 => {
            lox::run_prompt()?;
        }
        2 => {
            lox::run_file(Path::new(args[0].as_str()))?;
        }
        _ => {
            eprintln!("Usage: rlox [script]");
        }
    }

    Ok(())
}
