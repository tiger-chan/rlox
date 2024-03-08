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

    pub fn as_string(&self, src: &str) -> String {
        format!(
            "{} {}",
            self.token_type.as_string(src),
            &src[self.start..(self.start + self.length)]
        )
    }
}
