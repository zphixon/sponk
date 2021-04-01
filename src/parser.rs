use crate::prelude::Token;

pub enum Statement {
    Assign { name: Token, expression: Expression },
    AssignQuote { name: Token, expression: Expression },
    Expression(Expression),
}

pub enum Expression {
    Ident {
        name: Token,
    },
    List {
        value: Vec<Token>,
    },
    MonadCall {
        op: Box<Expression>,
        lhs: Box<Expression>,
    },
    DyadCall {
        rhs: Box<Expression>,
        op: Box<Expression>,
        lhs: Box<Expression>,
    },
    Quote {
        colon: Token,
        expression: Box<Expression>,
    },
    Call {
        expression: Box<Expression>,
        colon: Token,
    },
    Spread {
        verb: Box<Expression>,
        slash: Token,
    },
    Lambda {
        left_brace: Token,
        expression: Box<Expression>,
    },
}

pub fn parse() -> Option<Statement> {
    None
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn parse1() {
        let s = Scanner::new("x+y");
        let v: Vec<_> = s.collect();
        v.into_iter()
            .zip(
                vec![
                    Token::no_span(TokenKind::Ident, "x"),
                    Token::no_span(TokenKind::Builtin, "+"),
                    Token::no_span(TokenKind::Ident, "y"),
                ]
                .into_iter(),
            )
            .map(|(a, b)| assert!(a.compare_no_span(b)))
            .for_each(drop);
    }

    #[test]
    fn parse2() {
        let _src = "a = 1 + 2";
        let _a = Statement::Assign {
            name: Token::no_span(TokenKind::Ident, "a"),
            expression: Expression::DyadCall {
                rhs: Box::new(Expression::List {
                    value: vec![Token::no_span(TokenKind::Int(1), "1")],
                }),
                op: Box::new(Expression::Ident {
                    name: Token::no_span(TokenKind::Ident, "+"),
                }),
                lhs: Box::new(Expression::List {
                    value: vec![Token::no_span(TokenKind::Int(2), "2")],
                }),
            },
        };
    }
}
