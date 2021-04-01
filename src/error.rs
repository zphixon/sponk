use crate::prelude::Span;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("Syntax error: {why}\nat {span}")]
    SyntaxError { why: anyhow::Error, span: Span },
    #[error("Invalid unicode sequence at {span}")]
    InvalidUnicode { span: Span },
    #[error("Unterminated string starting at {span}")]
    UnterminatedString { span: Span },
    #[error("Unknown escape code {code}")]
    UnknownEscapeCode { code: String },
}
