//! scanner

use crate::prelude::SponkError;

/// is user/builtin identifier
fn id(c: u8) -> bool {
    !c.is_ascii_punctuation() && !c.is_ascii_whitespace()
}

/// token kind
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenKind {
    /// left paren
    LeftParen,
    /// right paren
    RightParen,
    /// left arg
    LeftArg,
    /// right arg
    RightArg,
    /// left brace
    LeftBrace,
    /// right brace
    RightBrace,
    /// identifier
    Ident,
    /// colon equal
    ColonEqual,
    /// equal
    Equal,
    /// colon
    Colon,
    /// number
    Number,
    /// string
    String,
    /// eof
    EOF,
}

/// token
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Token<'a> {
    /// kind
    kind: TokenKind,
    /// lexeme
    lexeme: &'a str,
}

impl Token<'_> {
    /// new
    pub(crate) fn new(kind: TokenKind, lexeme: &str) -> Token<'_> {
        Token { kind, lexeme }
    }
}

/// scanner
#[derive(Debug)]
pub struct Scanner<'a> {
    /// beginning
    beginning: usize,
    /// current
    current: usize,
    /// source
    source: &'a [u8],
}

impl<'a> Scanner<'a> {
    /// new
    pub(crate) fn new(s: &str) -> Scanner<'_> {
        Scanner {
            beginning: 0,
            current: 0,
            source: s.as_bytes(),
        }
    }

    /// next token
    pub(crate) fn next_token(&mut self) -> Result<Token<'a>, SponkError> {
        if self.at_end() {
            Ok(Token::new(TokenKind::EOF, ""))
        } else {
            self.slurp_whitespace();
            self.beginning = self.current;
            Ok(Token::new(
                match self.advance() {
                    b'\0' => return Ok(Token::new(TokenKind::EOF, "")),
                    b'\'' => self.string(),

                    b':' => match self.peek() {
                        b'=' => {
                            self.advance();
                            Ok(TokenKind::ColonEqual)
                        }
                        _ => Ok(TokenKind::Colon),
                    },

                    b'=' => Ok(TokenKind::Equal),

                    b'(' => Ok(TokenKind::LeftParen),
                    b')' => Ok(TokenKind::RightParen),

                    b'L' => Ok(TokenKind::LeftArg),
                    b'R' => Ok(TokenKind::RightArg),

                    b'{' => Ok(TokenKind::LeftBrace),
                    b'}' => Ok(TokenKind::RightBrace),

                    c if c.is_ascii_digit() => self.number(),
                    c if c.is_ascii_punctuation() => Ok(self.operator()),
                    _ => Ok(self.ident()),
                }?,
                self.lexeme()?,
            ))
        }
    }

    /// string
    fn string(&mut self) -> Result<TokenKind, SponkError> {
        while !self.at_end() && self.peek() != b'\'' {
            if self.peek() == b'\\' {
                self.advance();
                if self.peek() != b'\'' {
                    return Err(SponkError::UnknownEscapeCode);
                }
            }
            self.advance();
        }
        if self.at_end() {
            Err(SponkError::UnterminatedString)
        } else {
            self.advance();
            Ok(TokenKind::String)
        }
    }

    /// builtin op
    fn operator(&mut self) -> TokenKind {
        while self.peek() == b'.' {
            self.advance();
        }
        TokenKind::Ident
    }

    /// identifier
    fn ident(&mut self) -> TokenKind {
        while id(self.peek()) && !self.at_end() {
            self.advance();
        }
        while self.peek() == b'.' {
            self.advance();
        }
        TokenKind::Ident
    }

    /// number
    fn number(&mut self) -> Result<TokenKind, SponkError> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == b'.' {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
            self.lexeme()?.parse::<f64>()?;
        } else {
            self.lexeme()?.parse::<i64>()?;
        }
        Ok(TokenKind::Number)
    }

    /// slurp whitespace
    fn slurp_whitespace(&mut self) {
        while self.peek().is_ascii_whitespace() {
            self.advance();
        }
    }

    /// at end
    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// advance
    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source.get(self.current - 1).copied().unwrap_or(b'\0')
    }

    /// peek
    fn peek(&mut self) -> u8 {
        if self.at_end() {
            b'\0'
        } else {
            self.source[self.current]
        }
    }

    /// lexeme
    fn lexeme(&self) -> Result<&'a str, SponkError> {
        Ok(std::str::from_utf8(
            &self.source[self.beginning..self.current],
        )?)
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token().ok().filter(|t| t.kind != TokenKind::EOF)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn scan1() {
        let mut s = Scanner::new("[[::[]]::]");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftArg, "[")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftArg, "[")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftArg, "[")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightArg, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightArg, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightArg, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan2() {
        let mut s = Scanner::new("{} :{}]:][:{]:}}{]:");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBrace, "}")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBrace, "}")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightArg, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightArg, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftArg, "[")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightArg, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBrace, "}")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBrace, "}")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightArg, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
    }

    #[test]
    fn scan3() {
        let mut s = Scanner::new("                 1");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Number, "1")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan4() {
        let mut s = Scanner::new("{223}");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Number, "223")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBrace, "}")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan5() {
        let mut s = Scanner::new("] ]:3.14:{  ");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightArg, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightArg, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Number, "3.14")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan6() {
        let mut s = Scanner::new("i:ii i 32.4:i");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "i")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "ii")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "i")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Number, "32.4")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "i")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan7() {
        let mut s = Scanner::new("  'hello \\' world'");
        assert_eq!(
            s.next_token(),
            Ok(Token::new(TokenKind::String, "'hello \\' world'"))
        );
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan8() {
        let mut s = Scanner::new("]} [{ ]}[{'}:heiojewojoije' }{} {  }  {[} }   :] [: :]}[]['hello '][ ]:{[}[:]   }[:{]:}  ");
        let mut i = 0;
        while s.next().is_some() {
            i += 1;
        }
        assert_eq!(i, 46);
    }

    #[test]
    fn scan9() {
        let mut s = Scanner::new("{i3289:jeiwe328 38.3");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "i3289")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "jeiwe328")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Number, "38.3")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan10() {
        let mut s = Scanner::new("amp:=[:[]:[:]");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "amp")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::ColonEqual, ":=")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftArg, "[")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftArg, "[")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightArg, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftArg, "[")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Colon, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightArg, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan11() {
        let v: Vec<_> = Scanner::new("x := 1 2 3 4 5").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["x", ":=", "1", "2", "3", "4", "5"]);
        let v: Vec<_> = Scanner::new("y := 6 7 8 9 10").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["y", ":=", "6", "7", "8", "9", "10"]);
        let v: Vec<_> = Scanner::new("x + y").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["x", "+", "y"]);
        let v: Vec<_> = Scanner::new("#$x").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["#", "$", "x"]);
        let v: Vec<_> = Scanner::new("{]+]}x").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["{", "]", "+", "]", "}", "x"]);
        let v: Vec<_> = Scanner::new("{1+]}(f 1 2 3 4 5)")
            .map(|t| t.lexeme)
            .collect();
        assert_eq!(
            v,
            vec!["{", "1", "+", "]", "}", "(", "f", "1", "2", "3", "4", "5", ")"]
        );
        let v: Vec<_> = Scanner::new("amp:=[:[ ]: [:]").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["amp", ":=", "[", ":", "[", "]", ":", "[", ":", "]"]);
    }

    #[test]
    fn scan12() {
        let v: Vec<_> = Scanner::new("+...+~$#@*-*::").map(|t| t.lexeme).collect();
        assert_eq!(
            v,
            vec!["+...", "+", "~", "$", "#", "@", "*", "-", "*", ":", ":"]
        );
    }

    #[test]
    fn scan13() {
        let mut s = Scanner::new("+.. -. x....");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "+..")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "-.")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "x....")));
    }

    #[test]
    fn scan14() {
        let v: Vec<_> = Scanner::new("x+y").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["x", "+", "y"]);
    }
}
