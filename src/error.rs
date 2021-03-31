//! error

/// error
#[derive(Debug, PartialEq)]
pub enum SponkError {
  /// syntax error
  SyntaxError,
  /// invalid unicode
  InvalidUnicode,
  /// unterminated string
  UnterminatedString,
  /// unknown escape code
  UnknownEscapeCode,
}

impl From<std::num::ParseFloatError> for SponkError {
  fn from(_: std::num::ParseFloatError) -> SponkError {
    SponkError::SyntaxError
  }
}

impl From<std::num::ParseIntError> for SponkError {
  fn from(_: std::num::ParseIntError) -> SponkError {
    SponkError::SyntaxError
  }
}

impl From<std::str::Utf8Error> for SponkError {
  fn from(_: std::str::Utf8Error) -> SponkError {
    SponkError::InvalidUnicode
  }
}

impl std::fmt::Display for SponkError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
      SponkError::SyntaxError => "syntax error",
      SponkError::InvalidUnicode => "invalid unicode",
      SponkError::UnterminatedString => "unterminated string",
      SponkError::UnknownEscapeCode => "unknown escape code",
    })
  }
}
