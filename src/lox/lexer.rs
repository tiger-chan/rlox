use crate::string_reader::StringReader;

use super::grammar;
use super::Token;

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
    src: &'a str,
}

#[derive(Debug, Default)]
struct LexerReport {
    has_error: bool,
    tokens: Vec<Token>,
    pos: usize,
}

impl grammar::Reporter for LexerReport {
    fn append(&mut self, sub_ctx: &StringReader, tt: super::TokenType) {
        let token = Token::new(tt, self.pos, sub_ctx.pos - self.pos);
        self.tokens.push(token)
    }

    fn report(&mut self, line: u32, location: &str, msg: &str) {
        eprintln!("[line {line}] Error {location}:{msg}");
        self.has_error = true;
    }
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src }
    }

    pub fn tokenize(&mut self) -> Result<Tokens, TokenizationError> {
        let mut ctx = StringReader::new(self.src);
        let mut reporter = LexerReport::default();
        while !ctx.is_eof() {
            reporter.pos = ctx.pos;
            let mut inner_ctx = ctx;
            grammar::scan(&mut inner_ctx, &mut reporter);
            ctx = inner_ctx;
        }

        println!("Completed tokenize: length is : {}", reporter.tokens.len());
        for x in reporter.tokens.iter() {
            println!("Token: {:?}", x);
        }

        Ok(Tokens {
            tokens: reporter.tokens,
        })
    }
}

#[derive(Debug, Default)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}
