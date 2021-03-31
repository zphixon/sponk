//! array

/// element
#[derive(Debug, PartialEq)]
pub enum Element {
  /// array
  Array(Array),
  /// int
  Int(i64), // TODO: f64, bigint, etc
}

/// array
#[derive(Debug, PartialEq)]
pub struct Array {
  /// size
  pub size: Vec<usize>,
  /// data
  pub data: Vec<Element>,
}
