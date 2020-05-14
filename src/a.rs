//! array

use super::*;

/// element
#[derive(Debug, PartialEq)]
pub(crate) enum El {
  A(A),
  N(i64), // TODO: f64, bigint, etc
}

/// array
#[derive(Debug, PartialEq)]
pub(crate) struct A {
  pub(crate) d: Vec<U>,
  pub(crate) a: Vec<El>,
}
