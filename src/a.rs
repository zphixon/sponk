use super::*;

#[derive(Debug, PartialEq)]
pub(crate) enum E {
  A(A),
  N(i64), // TODO: f64, bigint, etc
}

#[derive(Debug, PartialEq)]
pub(crate) struct A {
  pub(crate) d: Vec<U>,
  pub(crate) a: Vec<E>,
}
