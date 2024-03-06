use crate::string_reader::StringReader;

use super::grammar;
use super::Token;
use super::TokenType;

#[derive(Debug)]
pub enum TokenizationError {}

impl std::fmt::Display for TokenizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tokenization Error")
    }
}

impl std::error::Error for TokenizationError {}

#[derive(Debug, Default)]
pub struct Lexer<'a> {
    has_error: bool,
    tokens: Vec<Token>,
    src: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            ..Default::default()
        }
    }

    pub fn tokenize(&mut self) -> Result<Tokens, TokenizationError> {
        let mut ctx = StringReader::new(self.src);
        while !ctx.is_eof() {
            let mut inner_ctx = ctx;
            let result = grammar::scan(&mut inner_ctx, |sub_ctx, tt| {
                let token = Token::new(tt, ctx.pos, sub_ctx.pos - ctx.pos);
                self.tokens.push(token)
            });

            if result.is_none() {
                // Not sure yet
                self.report(ctx.line, "", "unexpected character.");
            }
            ctx = inner_ctx;
        }
        Ok(Tokens::default())
    }

    fn report(&mut self, line: u32, location: &str, msg: &str) {
        eprintln!("[line {line}] Error {location}:{msg}");
        self.has_error = true;
    }
}

#[derive(Debug, Default)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}
