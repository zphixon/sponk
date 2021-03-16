//! parser

use super::*;

/// syntax tree
pub(crate) enum St<'a> {
  /// anon function
  Dvn(Vec<T<'a>>),
}

pub(crate) fn p() -> O<St<'static>> {
  N
}

#[cfg(test)]
mod t {
  use super::*;
  #[test]
  fn p() {
    let mut s = Sc::n("x+y");
    let v: Vec<_> = s.collect();
    assert_eq!(v, vec![T::n(Tk::I(0), "x"), T::n(Tk::I(0), "+"), T::n(Tk::I(0), "y")]);
  }
}

// x =. 1 2 3 4 5
// y =. 6 7 8 9 10
// x + y
// # $ x
// {] + ]} x
// {1+]} (f 1 2 3 4 5)
// amp :=: [:[ ]: [:]
