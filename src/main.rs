mod ast_printer;
mod lox;
mod string_reader;

use ast_printer::AstPrinter;
use lox::{Binary, Expr, Grouping, Literal, Unary, Value, Visitor};
use string_reader::StringReader;

use std::{env, error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let expr = Expr::Binary(Binary {
        left: Box::new(Expr::Unary(Unary {
            op: lox::UnaryOp::Minus,
            right: Box::new(Expr::Literal(Literal {
                value: Value::Num(1.0),
            })),
        })),
        op: lox::BinaryOp::Mul,
        right: Box::new(Expr::Grouping(Grouping {
            expression: Box::new(Expr::Literal(Literal {
                value: Value::Num(45.67),
            })),
        })),
    });

    let printer = AstPrinter {};
    println!("{}", printer.accept(&expr));

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
