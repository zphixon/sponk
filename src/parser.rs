//! parser

use crate::prelude::Token;

/// syntax tree
pub enum Statement<'a> {
  /// assign
  Assign {
    /// name
    name: Token<'a>,
    /// expression
    expression: Expression<'a>,
  },
  /// assign quote
  AssignQuote {
    /// name
    name: Token<'a>,
    /// expression
    expression: Expression<'a>,
  },
  Expression(Expression<'a>),
}

pub enum Expression<'a> {
  /// identifier
  Ident {
    /// name
    name: Token<'a>,
  },
  /// list
  List {
    /// value
    value: Vec<Token<'a>>,
  },
  /// monad call
  MonadCall {
    /// operator
    op: Box<Expression<'a>>,
    /// operand
    lhs: Box<Expression<'a>>,
  },
  /// dyad call
  DyadCall {
    /// rhs operand
    rhs: Box<Expression<'a>>,
    /// operator
    op: Box<Expression<'a>>,
    /// lhs operand
    lhs: Box<Expression<'a>>,
  },
  /// quote
  Quote {
    /// colon
    colon: Token<'a>,
    /// expression
    expression: Box<Expression<'a>>,
  },
  /// call
  Call {
    expression: Box<Expression<'a>>,
    colon: Token<'a>,
  },
  /// spread
  Spread {
    /// verb
    verb: Box<Expression<'a>>,
    /// slash
    slash: Token<'a>,
  },
  /// anon function
  Lambda {
    /// left brace
    left_brace: Token<'a>,
    /// expr
    expression: Box<Expression<'a>>
  },
}

pub fn parse() -> Option<Statement<'static>> {
  None
}

#[cfg(test)]
mod test {
  use crate::prelude::*;

  #[test]
  fn parse1() {
    let s = Scanner::new("x+y");
    let v: Vec<_> = s.collect();
    assert_eq!(v, vec![Token::new(TokenKind::Ident, "x"), Token::new(TokenKind::Ident, "+"), Token::new(TokenKind::Ident, "y")]);
  }

  #[test]
  fn parse2() {
    let _src = "a = 1 + 2";
    let _a = Statement::Assign {
      name: Token::new(TokenKind::Ident, "a"),
      expression: Expression::DyadCall {
        rhs: Box::new(Expression::List {
          value: vec![Token::new(TokenKind::Number, "1")],
        }),
        op: Box::new(Expression::Ident {
          name: Token::new(TokenKind::Ident, "+")
        }),
        lhs: Box::new(Expression::List {
          value: vec![Token::new(TokenKind::Number, "2")]
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
