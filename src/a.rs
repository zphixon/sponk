//! array

use super::*;

/// element
#[derive(Debug, PartialEq)]
pub(crate) enum El {
  /// array
  A(A),
  /// int
  N(i64), // TODO: f64, bigint, etc
}

/// array
#[derive(Debug, PartialEq)]
pub(crate) struct A {
  /// size
  pub(crate) s: Vec<U>,
  /// data
  pub(crate) d: Vec<El>,
}
