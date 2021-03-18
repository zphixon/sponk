//! parser

use super::*;

/// syntax tree
pub(crate) enum St<'a> {
  /// assign
  As {
    n: T<'a>,
    op: T<'a>,
    ex: Ex<'a>,
  },
  /// expression
  Ex(Ex<'a>),
}

pub(crate) enum Ex<'a> {
  /// identifier
  I {
    n: T<'a>,
  },
  /// list
  L {
    v: Vec<T<'a>>,
  },
  /// monad call
  M {
    op: Box<Ex<'a>>,
    rhs: Box<Ex<'a>>,
  },
  /// dyad call
  D {
    lhs: Box<Ex<'a>>,
    op: Box<Ex<'a>>,
    rhs: Box<Ex<'a>>,
  },
  /// monad operator
  Om {
    op: T<'a>,
    rhs: Box<Ex<'a>>,
  },
  /// dyad operator
  Od {
    lhs: Box<Ex<'a>>,
    op: T<'a>,
    rhs: Box<Ex<'a>>,
  },
  /// quote
  Q {
    l: T<'a>,
    ex: Box<Ex<'a>>,
    r: T<'a>,
  },
  /// paren
  P {
    l: T<'a>,
    ex: Box<Ex<'a>>,
    r: T<'a>,
  },
}

pub(crate) fn p(_a: &str) -> O<St<'_>> {
  N
}

#[cfg(test)]
mod t {
  use super::*;

  #[test]
  fn p() {
    let mut s = Sc::n("x+y");
    let v: Vec<_> = s.collect();
    assert_eq!(v, vec![T::n(Tk::I, "x"), T::n(Tk::O, "+"), T::n(Tk::I, "y")]);
  }

  #[test]
  fn p2() {
    let src = "a = 1 + 2";
    let _a1 = super::p(src);
    let _a2 = St::As {
      n: T::n(Tk::I, "a"),
      op: T::n(Tk::O, "="),
      ex: Ex::D {
        lhs: Box::new(Ex::L {
          v: vec![T::n(Tk::Dg, "1")],
        }),
        op: Box::new(Ex::I {
          n: T::n(Tk::I, "+")
        }),
        rhs: Box::new(Ex::L {
          v: vec![T::n(Tk::Dg, "2")]
        }),
      },
    };
  }

  #[test]
  fn p3() {
    let _a = St::As {
      n: T::n(Tk::I, "double"),
      op: T::n(Tk::O, "=."),
      ex: Ex::Q {
        l: T::n(Tk::Lb, "{"),
        ex: Box::new(Ex::Od {
          lhs: Box::new(Ex::L {
            v: vec![T::n(Tk::Dg, "2")],
          }),
          op: T::n(Tk::O, "*"),
          rhs: Box::new(Ex::I {
            n: T::n(Tk::O, "]"),
          }),
        }),
        r: T::n(Tk::Rb, "}"),
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
