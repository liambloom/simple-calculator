use std::{error::Error, fmt::{self, Formatter, Display}, sync::atomic::{AtomicUsize, Ordering}, iter::Peekable, num::ParseIntError};

#[derive(Debug, Clone, Copy)]
struct Location {
    begin: usize,
    len: usize,
}

#[derive(Debug, Clone)]
pub struct TokenizationError {
    location: Location,
    kind: TokenizationErrorKind,
}

#[derive(Debug, Clone)]
pub enum TokenizationErrorKind {
    NoSuchPunct(String),
    ParseIntError(ParseIntError)
}

impl Error for TokenizationError {}

impl Display for TokenizationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} at {}", self.kind, self.location.begin)
    }
}

struct RawToken<'a> {
    location: Location,
    content: &'a str,
}

pub struct Token {
    location: Location,
    content: TokenContent,
}

pub enum TokenContent {
    /// Any punctuation mark/operator
    Punct(Punct),

    /// Word is any free floating word that is part of the code. It
    /// may be an identifier or a keyword
    Word(Ident),

    Number(u64),
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
                location: value.location,
                kind: TokenizationErrorKind::NoSuchPunct(value.content.to_string()),
            }),
        }
    }
}

pub struct Ident;

pub fn tokenize(mut stream: Peekable<impl Iterator<Item = char>>) -> Vec<Token> {
    let mut pos = 0;
    let mut tokens = Vec::new();

    while let Some(c) = stream.next() {
        if let Ok(punct) = (&RawToken {
                content: &String::from(c),
                location: Location { begin: pos, len: 1 }
            }).try_into()
        {
            tokens.push(Token { 
                content: TokenContent::Punct(punct), 
                location: Location { begin: pos, len: 1 }
            });

            pos += 1;
        }
        else if c.is_numeric() {
            let mut num_str = String::from(c);
            while let Some(n) = stream.peek() {
                if n.is_numeric() {
                    num_str.push(stream.next().unwrap());
                }
                else {
                    break;
                }
            }

            let location = Location { begin: pos, len: num_str.len() };

            tokens.push(Token {
                content: TokenContent::Number(u64::from_str_radix(&num_str, 10)
                    .expect("Unexpected error when parsing a number")),
                location,
            })
        }
        else if c.is_ascii_alphabetic() {
            todo!("check for ident")
        }
    }

    todo!("Make sure that mistakes are fixed (e.g. if a number beginning
        with a decimal point was read as Punct::Period followed by a number,
        fix that");

    tokens
}
