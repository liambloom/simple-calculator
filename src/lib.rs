pub mod tokenize;
pub mod ast;
pub mod error;
mod native_functions;

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
}

impl Default for Location {
    fn default() -> Self {
        Self { begin: 0, len: 1 }
    }
}

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