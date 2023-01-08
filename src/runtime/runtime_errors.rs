use crate::tokenize::Ident;
use crate::LocatableContent;

use super::Location;

pub trait RuntimeError {
  fn to_string(&self) -> String;
  fn location(&self) -> Location;
}

pub struct ResolutionError(LocatableContent<Ident>);

impl ResolutionError {
  pub fn new(ident: LocatableContent<Ident>) -> Self {
    Self(ident)
  }
}

impl RuntimeError for ResolutionError {
  fn to_string(&self) -> String {
    format!(r#"Could not resolve "{}" @ {}"#, self.0.content(), self.0.location()).into()
  }

  fn location(&self) -> Location {
      self.0.location
  }
}

pub struct NonReturingFunctionError(Location);

impl NonReturingFunctionError {
  pub fn new(location: Location) -> Self {
    Self(location)
  }
}

impl RuntimeError for NonReturingFunctionError {
  fn to_string(&self) -> String {
    format!(r#"Non-returning cannot be used here @ "{}"#, self.0).into()
  }

  fn location(&self) -> Location {
      self.0
  }
}
