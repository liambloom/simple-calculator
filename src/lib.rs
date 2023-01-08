use std::collections::Bound;
use std::fmt::{self, Formatter, Display};
use std::ops::{Range, RangeBounds, RangeInclusive};

pub mod tokenize;
pub mod ast;
pub mod error;
pub mod runtime;

// Why not just use Range? It doesn't implement Copy, because this is not what it's meant for
// (see https://www.reddit.com/r/rust/comments/rrgxr0/a_critique_of_rusts_range_types/?utm_source=share&utm_medium=web2x&context=3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    /// The (exclusive) end of the location
    pub fn end(&self) -> usize {
        self.begin + self.len
    }
}

impl Default for Location {
    fn default() -> Self {
        Self { begin: 0, len: 1 }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.begin, self.begin + self.len)
    }
}

impl From<Range<usize>> for Location {
    fn from(r: Range<usize>) -> Self {
        Self {
            begin: r.start,
            len: r.end - r.start,
        }
    }
}

impl From<RangeInclusive<usize>> for Location {
    fn from(r: RangeInclusive<usize>) -> Self {
        Self {
            begin: *r.start(),
            len: r.end() - r.start() + 1,
        }
    }
}

impl From<Range<Location>> for Location {
    fn from(r: Range<Location>) -> Self {
        Self {
            begin: r.start.begin,
            len: r.end.begin - r.start.begin,
        }
    }
}

impl From<RangeInclusive<Location>> for Location {
    fn from(r: RangeInclusive<Location>) -> Self {
        Self {
            begin: r.start().begin,
            len: r.end().begin + r.end().len - r.start().begin,
        }
    }
}

// impl RangeBounds<usize> for Location {
//     fn start_bound(&self) -> Bound<&usize> {
//         Bound::Included(&self.begin)
//     }
//
//     fn end_bound(&self) -> Bound<&usize> {
//         Bound::Excluded(&(self.begin + self.len))
//     }
// }


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocatableContent<T> {
    location: Location,
    content: T,
}

impl<T> LocatableContent<T> {
    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn content(&self) -> &T {
        &self.content
    }

    pub fn new(content: T, location: Location) -> Self {
        Self { content, location }
    }
}