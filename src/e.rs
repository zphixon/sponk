//! error

use std::convert::From as F;
use std::num::{ParseIntError as Pie, ParseFloatError as Pfe};

/// error
#[derive(Debug, PartialEq)]
pub enum E {
  Syntax,
  Unicode,
  UnterminatedString,
  UnknownEscapeCode,
}

impl F<Pfe> for E {
  fn from(_: Pfe) -> E {
    E::Syntax
  }
}

impl F<Pie> for E {
  fn from(_: Pie) -> E {
    E::Syntax
  }
}

impl F<std::str::Utf8Error> for E {
  fn from(_: std::str::Utf8Error) -> E {
    E::Unicode
  }
}
