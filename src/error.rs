use std::{error::Error, fmt::{self, Display, Formatter}};

use crate::Location;

#[derive(Debug, Copy, Clone)]
struct UnknownLocation;

#[derive(Debug, Clone)]
pub struct CompilationError {
    location: Location,
    kind: CompilationErrorKind,
}

#[derive(Debug, Clone)]
pub enum CompilationErrorKind {
    /// A character that is not recognized by the compiler
    UnrecognizedCharacter,

    /// A number with too much precision (not used I think)
    TooPreciseNumber,

    /// A number contains multiple decimal points
    TwoDecimalPoints,

    /// The number of opening delimiters does not equal the number
    /// of closing delimiters. E.g. `(foo` or `bar}`
    UnmatchedDelimiter,

    /// The opening and closing delimiters do not match. E.g. `(foo}`
    MismatchedDelimiter,
}

impl CompilationError {
    pub fn new(location: Location, kind: CompilationErrorKind) -> Self {
        Self { location, kind }
    }
}



// impl From<PoisonError<_>> for TokenizationError {
//     fn from(e: PoisonError<_>) -> Self {
//         Self {
//             location: None,
//             kind: 
//         }
//     }
// }

impl Error for CompilationError {}

impl Display for CompilationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} at {}", self.kind, self.location.begin)
    }
}