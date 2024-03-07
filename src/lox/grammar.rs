use crate::StringReader;

use super::TokenType;
type TT = TokenType;

pub trait Reporter {
    fn append(&mut self, ctx: &StringReader, tt: TokenType);
    fn report(&mut self, line: u32, location: &str, msg: &str);
}

fn consume(ctx: &mut StringReader, tt: TokenType) -> Option<TokenType> {
    ctx.read();
    Some(tt)
}

fn consume_match(
    ctx: &mut StringReader,
    c: char,
    select: TokenType,
    otherwise: TokenType,
) -> Option<TokenType> {
    ctx.read();
    match ctx.peek() {
        x if x == c => consume(ctx, select),
        _ => Some(otherwise),
    }
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

fn eof(ctx: &mut StringReader, reporter: &mut dyn Reporter) -> Option<()> {
    if ctx.is_eof() || ctx.peek() == '\0' {
        consume(ctx, TT::Eof).map(|x| {
            reporter.append(ctx, x);
        })
    } else {
        None
    }
}

fn unexpected(ctx: &mut StringReader, reporter: &mut dyn Reporter) -> Option<()> {
    reporter.report(ctx.line, "", "Unexpected character.");
    None
}

fn whitespace(ctx: &mut StringReader) -> Option<()> {
    match ctx.peek() {
        ' ' | '\r' | '\t' => {
            ctx.read();
            None
        }
        '\n' => {
            ctx.read();
            ctx.ln();
            Some(())
        }
        _ => None,
    }
}

fn single(ctx: &mut StringReader, reporter: &mut dyn Reporter) -> Option<()> {
    let token = match ctx.peek() {
        '(' => consume(ctx, TT::LeftParen),
        ')' => consume(ctx, TT::RightParen),
        '{' => consume(ctx, TT::LeftBrace),
        '}' => consume(ctx, TT::RightBrace),
        ',' => consume(ctx, TT::Comma),
        '.' => consume(ctx, TT::Dot),
        '-' => consume(ctx, TT::Minus),
        '+' => consume(ctx, TT::Plus),
        ';' => consume(ctx, TT::Semicolon),
        '*' => consume(ctx, TT::Star),
        _ => None,
    };

    token.map(|x| {
        reporter.append(ctx, x);
    })
}

fn logical_op(ctx: &mut StringReader, reporter: &mut dyn Reporter) -> Option<()> {
    let token = match ctx.peek() {
        '!' => consume_match(ctx, '=', TT::BangEqual, TT::Bang),
        '=' => consume_match(ctx, '=', TT::Equality, TT::Assignment),
        '>' => consume_match(ctx, '=', TT::GreaterEqual, TT::Greater),
        '<' => consume_match(ctx, '=', TT::LessEqual, TT::Less),
        _ => None,
    };

    token.map(|x| {
        reporter.append(ctx, x);
    })
}

fn slash_comments(ctx: &mut StringReader, reporter: &mut dyn Reporter) -> Option<()> {
    match ctx.peek() {
        '/' => {
            ctx.read();
            match ctx.peek() {
                '/' => {
                    ctx.read();
                    while ctx.peek() != '\n' && !ctx.is_eof() {
                        ctx.read();
                    }
                }
                // '*' could be checked and would keep checking until it found
                // a `*/` sequence
                _ => reporter.append(ctx, TT::Slash),
            }
            Some(())
        }
        _ => None,
    }
}

fn string(ctx: &mut StringReader) -> Option<TokenType> {
    ctx.read();
    let start = ctx.pos + 1;
    while ctx.peek() == '"' && !ctx.is_eof() {
        if ctx.peek() == '\n' {
            ctx.ln();
        }
        ctx.read();
    }

    if ctx.is_eof() {
        None
    } else {
        ctx.read();

        Some(TT::String {
            offset: start,
            len: (ctx.pos - start) - 1,
        })
    }
}

fn string_lit(ctx: &mut StringReader, reporter: &mut dyn Reporter) -> Option<()> {
    match ctx.peek() {
        '"' => string(ctx).map(|x| reporter.append(ctx, x)),
        _ => None,
    }
}

fn numeric(ctx: &mut StringReader) -> Option<TokenType> {
    let start = ctx.pos;
    ctx.read();
    while is_digit(ctx.peek()) {
        ctx.read();
    }

    let mut tmp_ctx = *ctx;
    if tmp_ctx.read() == '.' {
        is_digit(tmp_ctx.peek()).then(|| {
            while is_digit(tmp_ctx.peek()) {
                tmp_ctx.read();
            }
            *ctx = tmp_ctx;
        });
    }

    let number = &ctx.src[start..ctx.pos];

    Some(TokenType::Number(number.parse::<f32>().unwrap()))
}

fn numeric_lit(ctx: &mut StringReader, reporter: &mut dyn Reporter) -> Option<()> {
    match ctx.peek() {
        x if is_digit(x) => numeric(ctx).map(|x| reporter.append(ctx, x)),
        _ => None,
    }
}

fn identifier(ctx: &mut StringReader<'_>) -> (usize, usize) {
    let start = ctx.pos;
    ctx.read();
    while is_alpha_numeric(ctx.peek()) {
        ctx.read();
    }

    (start, ctx.pos)
}

fn identifiers_and_keywords(ctx: &mut StringReader, reporter: &mut dyn Reporter) -> Option<()> {
    match ctx.peek() {
        x if is_alpha(x) => {
            let (s, e) = identifier(ctx);
            let value = &ctx.src[s..e];
            match value {
                "and" => reporter.append(ctx, TT::And),
                "struct" => reporter.append(ctx, TT::Struct),
                "else" => reporter.append(ctx, TT::Else),
                "false" => reporter.append(ctx, TT::False),
                "for" => reporter.append(ctx, TT::For),
                "fn" => reporter.append(ctx, TT::Fn),
                "if" => reporter.append(ctx, TT::If),
                "nil" => reporter.append(ctx, TT::Nil),
                "or" => reporter.append(ctx, TT::Or),
                "print" => reporter.append(ctx, TT::Print),
                "return" => reporter.append(ctx, TT::Return),
                "super" => reporter.append(ctx, TT::Super),
                "self" => reporter.append(ctx, TT::This),
                "true" => reporter.append(ctx, TT::True),
                "var" => reporter.append(ctx, TT::Var),
                "while" => reporter.append(ctx, TT::While),
                _ => reporter.append(
                    ctx,
                    TT::Identifier {
                        offset: s,
                        len: e - s,
                    },
                ),
            }
            Some(())
        }
        _ => None,
    }
}

pub fn scan(ctx: &mut StringReader, reporter: &mut dyn Reporter) -> Option<()> {
    eof(ctx, reporter)
        .or_else(|| whitespace(ctx))
        .or_else(|| single(ctx, reporter))
        .or_else(|| logical_op(ctx, reporter))
        .or_else(|| slash_comments(ctx, reporter))
        .or_else(|| string_lit(ctx, reporter))
        .or_else(|| numeric_lit(ctx, reporter))
        .or_else(|| identifiers_and_keywords(ctx, reporter))
        .or_else(|| unexpected(ctx, reporter))
}
