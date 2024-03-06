mod grammar;
mod lexer;
mod token;
mod token_type;

pub use lexer::{Lexer, TokenizationError, Tokens};
pub use token::Token;
pub use token_type::TokenType;

use std::{error::Error, io::Write, path::Path};

pub fn run_prompt() -> Result<(), Box<dyn Error>> {
    use std::io::{stdin, stdout};

    let mut buf = String::new();
    loop {
        print!("> ");
        stdout().flush()?;
        buf.clear();
        match stdin().read_line(&mut buf) {
            Ok(r) => {
                if r == 0 || r == 1 {
                    println!("Quiting");
                    return Ok(());
                } else {
                    run(buf.as_str())?;
                }
            }
            Err(err) => {
                return Err(Box::new(err));
            }
        }
    }
}

pub fn run_file(path: &Path) -> Result<(), Box<dyn Error>> {
    println!("Run file {}", path.display());
    let contents = std::fs::read_to_string(path)?;
    run(contents.as_str())
}

fn run(buf: &str) -> Result<(), Box<dyn Error>> {
    let mut lexer = Lexer::new(buf);
    lexer.tokenize()?;
    Ok(())
}
