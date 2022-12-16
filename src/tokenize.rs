use std::{error::Error, fmt::{self, Formatter, Display}, sync::atomic::{AtomicUsize, Ordering}, iter::Peekable, num::ParseIntError};

mod primary;
mod secondary;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct LeadingZerosU64 {
    value: u64,
    zeros: u8,
}

impl LeadingZerosU64 {
    const ZERO: LeadingZerosU64 = LeadingZerosU64 {
        value: 0,
        zeros: 0,
    };

    pub fn value(&self) -> &u64 {
        &self.value
    }

    pub fn zeros(&self) -> &u8 {
        &self.zeros
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Location {
    begin: usize,
    len: usize,
}

impl Location {
    pub fn begin(&self) -> usize {
        self.begin
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[derive(Debug, Clone)]
pub struct TokenizationError {
    location: Location,
    kind: TokenizationErrorKind,
}

#[derive(Debug, Clone)]
pub enum TokenizationErrorKind {
    UnrecognizedCharacter,
    TooPreciseNumber,
    TwoDecimalPoints,
}

impl Error for TokenizationError {}

impl Display for TokenizationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} at {}", self.kind, self.location.begin)
    }
}
