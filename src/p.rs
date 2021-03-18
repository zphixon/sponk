//! parser

use super::*;

/// syntax tree
pub(crate) enum St<'a> {
  /// assign
  As {
    /// name
    n: T<'a>,
    /// expression
    e: Ex<'a>,
  },
  /// assign quote
  Aq {
    /// name
    n: T<'a>,
    /// expression
    e: Ex<'a>,
  },
  Ex(Ex<'a>),
}

pub(crate) enum Ex<'a> {
  /// identifier
  I {
    /// name
    n: T<'a>,
  },
  /// list
  L {
    /// value
    v: Vec<T<'a>>,
  },
  /// monad call
  M {
    /// operator
    op: Box<Ex<'a>>,
    /// operand
    lhs: Box<Ex<'a>>,
  },
  /// dyad call
  D {
    /// rhs operand
    rhs: Box<Ex<'a>>,
    /// operator
    op: Box<Ex<'a>>,
    /// lhs operand
    lhs: Box<Ex<'a>>,
  },
  /// quote
  Q {
    /// colon
    co: T<'a>,
    /// expression
    e: Box<Ex<'a>>,
  },
  /// call
  C {
    e: Box<Ex<'a>>,
    co: T<'a>,
  },
  /// spread
  Sp {
    /// verb
    v: Box<Ex<'a>>,
    /// slash
    s: T<'a>,
  },
  /// anon function
  Dfn {
    /// left brace
    lb: T<'a>,
    /// expr
    e: Box<Ex<'a>>
  },
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
    assert_eq!(v, vec![T::n(Tk::I, "x"), T::n(Tk::I, "+"), T::n(Tk::I, "y")]);
  }

  #[test]
  fn p2() {
    let src = "a = 1 + 2";
    let a = St::As {
      n: T::n(Tk::I, "a"),
      e: Ex::D {
        rhs: Box::new(Ex::L {
          v: vec![T::n(Tk::Dg, "1")],
        }),
        op: Box::new(Ex::I {
          n: T::n(Tk::I, "+")
        }),
        lhs: Box::new(Ex::L {
          v: vec![T::n(Tk::Dg, "2")]
        }),
      },
    };
  }
}

// x =. 1 2 3 4 5
// y =. 6 7 8 9 10
// x + y
// # $ x
// {] + ]} x
// {1+]} (f 1 2 3 4 5)
// amp :=: [:[ ]: [:]
