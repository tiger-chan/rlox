#[derive(Debug, Default, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Assignment,
    Equality,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier {
        offset: usize,
        len: usize,
    },
    String {
        offset: usize,
        len: usize,
    },
    Number(f32),

    // Keywords.
    And,
    Struct,
    Else,
    False,
    Fn,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    #[default]
    Eof,
}

impl TokenType {
    pub fn as_string(&self, src: &str) -> String {
        match self {
            TokenType::Identifier { offset, len } => {
                format!("Identifier {}", &src[*offset..(*offset + *len)])
            }
            TokenType::String { offset, len } => {
                format!("String '{}'", &src[*offset..(*offset + *len)])
            }
            TokenType::Number(v) => format!("Number {}", v),
            _ => format!("{:?}", self),
        }
    }
}
