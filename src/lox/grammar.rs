use crate::StringReader;

use super::TokenType;
type TT = TokenType;

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
        x if x == c => Some(select),
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

fn whitespace(ctx: &mut StringReader) -> Option<()> {
    match ctx.peek() {
        ' ' | '\r' | '\t' => {
            ctx.read();
            None
        }
        '\n' => {
            ctx.read();
            ctx.ln();
            None
        }
        _ => None,
    }
}

fn single<F>(ctx: &mut StringReader, apend_token: &mut F) -> Option<()>
where
    F: FnMut(&StringReader, TokenType),
{
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
        apend_token(ctx, x);
    })
}

fn logical_op<F>(ctx: &mut StringReader, apend_token: &mut F) -> Option<()>
where
    F: FnMut(&StringReader, TokenType),
{
    let token = match ctx.peek() {
        '!' => consume_match(ctx, '=', TT::BangEqual, TT::Bang),
        '=' => consume_match(ctx, '=', TT::Equality, TT::Assignment),
        '>' => consume_match(ctx, '=', TT::GreaterEqual, TT::Greater),
        '<' => consume_match(ctx, '=', TT::LessEqual, TT::Less),
        _ => None,
    };

    token.map(|x| {
        apend_token(ctx, x);
    })
}

fn slash_comments<F>(ctx: &mut StringReader, apend_token: &mut F) -> Option<()>
where
    F: FnMut(&StringReader, TokenType),
{
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
                _ => apend_token(ctx, TT::Slash),
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

fn string_lit<F>(ctx: &mut StringReader, append_token: &mut F) -> Option<()>
where
    F: FnMut(&StringReader, TokenType),
{
    match ctx.peek() {
        '"' => string(ctx).map(|x| append_token(ctx, x)),
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

fn numeric_lit<F>(ctx: &mut StringReader, append_token: &mut F) -> Option<()>
where
    F: FnMut(&StringReader, TokenType),
{
    match ctx.peek() {
        x if is_digit(x) => numeric(ctx).map(|x| append_token(ctx, x)),
        _ => None,
    }
}

fn identifier<'a>(ctx: &mut StringReader<'a>) -> (usize, usize) {
    let start = ctx.pos;
    ctx.read();
    while is_alpha_numeric(ctx.peek()) {
        ctx.read();
    }

    (start, ctx.pos)
}

fn identifiers_and_keywords<F>(ctx: &mut StringReader, append_token: &mut F) -> Option<()>
where
    F: FnMut(&StringReader, TokenType),
{
    match ctx.peek() {
        x if is_alpha(x) => {
            let (s, e) = identifier(ctx);
            let value = &ctx.src[s..e];
            match value {
                "and" => append_token(ctx, TT::And),
                "struct" => append_token(ctx, TT::Struct),
                "else" => append_token(ctx, TT::Else),
                "false" => append_token(ctx, TT::False),
                "for" => append_token(ctx, TT::For),
                "fn" => append_token(ctx, TT::Fn),
                "if" => append_token(ctx, TT::If),
                "nil" => append_token(ctx, TT::Nil),
                "or" => append_token(ctx, TT::Or),
                "print" => append_token(ctx, TT::Print),
                "return" => append_token(ctx, TT::Return),
                "super" => append_token(ctx, TT::Super),
                "self" => append_token(ctx, TT::This),
                "true" => append_token(ctx, TT::True),
                "var" => append_token(ctx, TT::Var),
                "while" => append_token(ctx, TT::While),
                _ => append_token(
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

pub fn scan<F>(ctx: &mut StringReader, mut append_token: F) -> Option<()>
where
    F: FnMut(&StringReader, TokenType),
{
    whitespace(ctx)
        .or_else(|| single(ctx, &mut append_token))
        .or_else(|| logical_op(ctx, &mut append_token))
        .or_else(|| slash_comments(ctx, &mut append_token))
        .or_else(|| string_lit(ctx, &mut append_token))
        .or_else(|| numeric_lit(ctx, &mut append_token))
        .or_else(|| identifiers_and_keywords(ctx, &mut append_token))
}
