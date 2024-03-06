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
    Identifier{ offset: usize, len: usize },
    String{ offset: usize, len: usize },
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
