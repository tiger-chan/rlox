mod lox;
mod string_reader;

use string_reader::StringReader;

use std::{env, error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
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
