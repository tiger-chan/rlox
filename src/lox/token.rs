use super::TokenType;

#[derive(Debug, Default)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
}

impl Token {
    pub fn new(ttype: TokenType, start: usize, length: usize) -> Self {
        Self {
            token_type: ttype,
            start,
            length,
        }
    }
}
