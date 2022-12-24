use std::{error::Error, fmt::{self, Formatter, Display}, sync::{atomic::{AtomicUsize, Ordering}, RwLock, Mutex, PoisonError}, iter::Peekable, num::ParseIntError, cell::RefCell, collections::HashMap, thread::{ThreadId, self}};
use crate::{Location, error::*, LocatableContent};
use num_rational::Rational64;

struct UnprocessedToken<'a>(&'a str);

pub type Token = LocatableContent<TokenContent>;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum TokenContent {
    /// Any punctuation mark/operator
    Punct(Punct),

    // Word is any free floating word that is part of the code. It
    // may be an identifier or a keyword
    //Word(Ident),

    Value(Value),
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum Value {
    Number(Rational64),
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
    // Period,
}

impl <'a> TryFrom<UnprocessedToken<'a>> for Punct {
    type Error = NoSuchPunct;

    fn try_from(value: UnprocessedToken<'a>) -> Result<Self, NoSuchPunct> {
        use Punct::*;

        match value.0 {
            "+" => Ok(Plus),
            "-" => Ok(Dash),
            "*" => Ok(Asterisk),
            "/" => Ok(Slash),
            "(" => Ok(OpenParenth),
            ")" => Ok(CloseParenth),
            // "." => Ok(Period),
            _ => Err(NoSuchPunct),
        }
    }
}

pub fn tokenize(stream: impl Iterator<Item = char>) -> Result<TokenStream, CompilationError> {
    Tokenizer::new().tokenize(stream.peekable())
}

// pub struct Ident;

pub type TokenStream = Vec<Token>;

pub struct Tokenizer {
    location: Location,
}

impl Tokenizer {
    fn new() -> Self {
        Self { location: Location::default() }
    }

    fn error(&self, kind: CompilationErrorKind) -> CompilationError {
        CompilationError::new(self.location, kind)
    }

    pub fn tokenize(&mut self, stream: Peekable<impl Iterator<Item = char>>) -> Result<TokenStream, CompilationError> {
        self.tokenize_inner(stream).map_err(|kind| self.error(kind))
    }

    fn tokenize_inner(&mut self, mut stream: Peekable<impl Iterator<Item = char>>) -> Result<TokenStream, CompilationErrorKind> {
        let mut tokens = Vec::new();

        while let Some(c) = stream.next() {
            self.location.len = 1;

            if c.is_numeric() || c == '.' && stream.peek().map_or(false, |c| c.is_numeric()) {
                let mut num_str = String::from(c);

                let mut point_has_passed = c == '.';

                while let Some(n) = stream.peek() {
                    self.location.len += 1;

                    if n.is_numeric() {
                        num_str.push(stream.next().unwrap());
                    }
                    else if n == &'.' {
                        if point_has_passed {
                            return Err(CompilationErrorKind::TwoDecimalPoints);
                        }
                        else {
                            num_str.push(stream.next().unwrap());
                            point_has_passed = true;
                        }
                    }
                    else {
                        self.location.len -= 1;
                        break;
                    }
                }

                // let location = Location { begin: pos, len: num_str.len() };
                // self.location = location;

                let value = match num_str.find('.') {
                    // The reason I don't just use Ratio::from_float(s.parse().unwrap()) is because of roundoff errors.
                    // For example, Ratio::from_float(0.3).unwrap() = 5,404,319,552,844,595/18,014,398,509,481,984
                    Some(index) => {
                        ( if index == 0 { Rational64::from_integer(0) }
                            else { Rational64::from_integer((&num_str[0..index]).parse().unwrap()) } )
                        + ( if index == num_str.len() - 1 { Rational64::from_integer(0) } 
                            else { Rational64::new
                                ((&num_str[index + 1..num_str.len()]).parse().unwrap(), 
                                10i64.pow((num_str.len() - index - 1).try_into().unwrap()) ) }
                        )
                    },
                    None => Rational64::from_integer(num_str.parse().unwrap()),
                };


                // let value = u64::from_str_radix(&num_str, 10)
                //     .expect("Unexpected error when parsing a number");
                // let zeros = if value == 0 { 0 } else {
                //     num_str.len() as u8 - ((value as f64).log10().ceil() as u8) - 1
                // };

                tokens.push(Token {
                    content: TokenContent::Value(Value::Number(value)),
                    location: self.location,
                });

                self.location.begin += num_str.len();
            }
            else if let Ok(punct) = UnprocessedToken(&String::from(c)).try_into()
            {
                tokens.push(Token { 
                    content: TokenContent::Punct(punct), 
                    location: self.location
                });

                self.location.begin += 1;
            }
            else if c == ' ' {
                self.location.begin += 1;
            }
            else {
                return Err(CompilationErrorKind::UnrecognizedCharacter);
            }
        }

        Ok(tokens)
    }
}



