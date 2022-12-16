use std::{iter::Peekable, str::FromStr};

use num_rational::Rational64;

use super::*;

struct UnprocessedToken<'a> {
    location: Location,
    content: &'a str,
}

pub struct PreToken {
    location: Location,
    content: PreTokenContent,
}

impl PreToken {
    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn content(&self) -> &PreTokenContent {
        &self.content
    }

    pub fn new(content: PreTokenContent, location: Location) -> Self {
        Self { content, location }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PreTokenContent {
    /// Any punctuation mark/operator
    Punct(Punct),

    // Word is any free floating word that is part of the code. It
    // may be an identifier or a keyword
    //Word(Ident),

    Number(LeadingZerosU64),
}

struct NoSuchPunct;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Punct {
    Plus,
    Dash,
    Asterisk,
    Slash,
    OpenParenth,
    CloseParenth,
    Period,
}

impl <'a> TryFrom<&UnprocessedToken<'a>> for Punct {
    type Error = NoSuchPunct;

    fn try_from(value: &UnprocessedToken<'a>) -> Result<Self, NoSuchPunct> {
        use Punct::*;

        match value.content {
            "+" => Ok(Plus),
            "-" => Ok(Dash),
            "*" => Ok(Asterisk),
            "/" => Ok(Slash),
            "(" => Ok(OpenParenth),
            ")" => Ok(CloseParenth),
            "." => Ok(Period),
            _ => Err(NoSuchPunct),
        }
    }
}

// pub struct Ident;

pub type PreTokenStream = Vec<PreToken>;

pub fn tokenize1(mut stream: Peekable<impl Iterator<Item = char>>) -> Result<PreTokenStream, TokenizationError> {
    let mut pos = 0;
    let mut tokens = Vec::new();

    while let Some(c) = stream.next() {
        if let Ok(punct) = (&UnprocessedToken {
                content: &String::from(c),
                location: Location { begin: pos, len: 1 }
            }).try_into()
        {
            tokens.push(PreToken { 
                content: PreTokenContent::Punct(punct), 
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

            let value = u64::from_str_radix(&num_str, 10)
                .expect("Unexpected error when parsing a number");
            let zeros = if value == 0 { 0 } else {
                num_str.len() as u8 - ((value as f64).log10().ceil() as u8) - 1
            };

            tokens.push(PreToken {
                content: PreTokenContent::Number(LeadingZerosU64 { value, zeros }),
                location,
            });

            pos += num_str.len();
        }
        else if c != ' ' {
            return Err(TokenizationError { 
                location: Location { begin: pos, len: 1 }, 
                kind: TokenizationErrorKind::UnrecognizedCharacter
            })
        }

        /*else if c.is_ascii_alphabetic() {
            todo!("check for ident")
        }*/
    }

    Ok(tokens)
}
