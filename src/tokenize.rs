use std::{error::Error, fmt::{self, Formatter, Display}, sync::atomic::{AtomicUsize, Ordering}};

#[derive(Debug, Clone)]
pub struct TokenizationError {
    begin: usize,
    end: usize,
    kind: TokenizationErrorKind,
}

#[derive(Debug, Clone)]
pub enum TokenizationErrorKind {
    NoSuchPunct(String),
}

impl Error for TokenizationError {}

impl Display for TokenizationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} at {}", self.kind, self.begin)
    }
}

struct RawToken<'a> {
    begin: usize,
    end: usize,
    content: &'a str
}

pub enum Token {
    Punct(Punct),
    Ident(Ident),
    Literal(Literal),
}

pub enum Punct {
    Plus,
    Dash,
    Asterisk,
    Slash,
    OpenParenth,
    CloseParenth,
    Period,
}

impl <'a> TryFrom<&RawToken<'a>> for Punct {
    type Error = TokenizationError;

    fn try_from(value: &RawToken<'a>) -> Result<Self, Self::Error> {
        use Punct::*;

        match value.content {
            "+" => Ok(Plus),
            "-" => Ok(Dash),
            "*" => Ok(Asterisk),
            "/" => Ok(Slash),
            "(" => Ok(OpenParenth),
            ")" => Ok(CloseParenth),
            "." => Ok(Period),
            _ => Err(TokenizationError {
                begin: value.begin,
                end: value.end,
                kind: TokenizationErrorKind::NoSuchPunct(value.content.to_string()),
            }),
        }
    }
}

pub struct Ident;

pub enum Literal {
    Number(i64, i8),
}

pub fn tokenize(mut stream: impl Iterator<Item = char>) -> Vec<Token> {
    let mut pos = 0;
    let mut tokens = Vec::new();

    while let Some(c) = stream.next() {
        if let Ok(punct) = (&RawToken {
            content: &String::from(c),
            begin: pos,
            end: pos,
        }).try_into()
        {
            tokens.push(Token::Punct(punct))
        }
        else if c.is_numeric() {
            todo!("if it is a number")
        }
        else if c.is_ascii_alphabetic() {
            todo!("check for ident")
        }

        pos += 1;
    }

    todo!("Make sure that mistakes are fixed (e.g. if a number beginning
        with a decimal point was read as Punct::Period followed by a number,
        fix that");

    tokens
}
