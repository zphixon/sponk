//! error

/// error
#[derive(Debug, PartialEq)]
pub enum E {
  Syntax,
  QuoteNonIdentifier,
  UnterminatedString,
  UnknownEscapeCode,
}

impl std::convert::From<std::num::ParseFloatError> for E {
  fn from(_: std::num::ParseFloatError) -> E {
    E::Syntax
  }
}

impl std::convert::From<std::num::ParseIntError> for E {
  fn from(_: std::num::ParseIntError) -> E {
    E::Syntax
  }
}
