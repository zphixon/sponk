//! error

use std::convert::From as F;
use std::fmt::{Display as D, Result as Fr, Formatter as Fm};

/// error
#[derive(Debug, PartialEq)]
pub enum E {
  /// syntax error
  S,
  /// invalid unicode
  Iu,
  /// unterminated string
  Us,
  /// unknown escape code
  Uec,
}

impl F<std::num::ParseFloatError> for E {
  fn from(_: std::num::ParseFloatError) -> E {
    E::S
  }
}

impl F<std::num::ParseIntError> for E {
  fn from(_: std::num::ParseIntError) -> E {
    E::S
  }
}

impl F<std::str::Utf8Error> for E {
  fn from(_: std::str::Utf8Error) -> E {
    E::Iu
  }
}

impl D for E {
  fn fmt(&self, f: &mut Fm) -> Fr {
    write!(f, "{}", match self {
      E::S => "syntax error",
      E::Iu => "invalid unicode",
      E::Us => "unterminated string",
      E::Uec => "unknown escape code",
    })
  }
}
