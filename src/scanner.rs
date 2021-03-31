//! scanner

use crate::prelude::SponkError;

use std::iter::Peekable;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

fn is_builtin(s: &str) -> bool {
    matches!(
        s,
        "-" | "`"
            | "="
            | "["
            | "]"
            | "\\"
            | ";"
            | ","
            | "."
            | "/"
            | "~"
            | "!"
            | "@"
            | "#"
            | "$"
            | "%"
            | "^"
            | "&"
            | "*"
            | "("
            | ")"
            | "_"
            | "+"
            | "{"
            | "}"
            | "|"
            | ":"
            | "\""
            | "<"
            | ">"
            | "?"
            | "⋄"
            | "¨"
            | "≤"
            | "≥"
            | "≠"
            | "∨"
            | "∧"
            | "×"
            | "÷"
            | "⍵"
            | "∊"
            | "⍴"
            | "↑"
            | "↓"
            | "⍳"
            | "○"
            | "←"
            | "→"
            | "⊢"
            | "⍺"
            | "⌈"
            | "⌊"
            | "∇"
            | "∆"
            | "∘"
            | "⎕"
            | "⍎"
            | "⍕"
            | "⊂"
            | "⊥"
            | "⊤"
            | "⍝"
            | "⍀"
            | "⌿"
            | "⌺"
            | "⌶"
            | "⍫"
            | "⍒"
            | "⍋"
            | "⌽"
            | "⍉"
            | "⊖"
            | "⍟"
            | "⍱"
            | "⌹"
            | "⍷"
            | "⍨"
            | "⍸"
            | "⍥"
            | "⍣"
            | "⍞"
            | "⍬"
            | "⊣"
            | "⍤"
            | "⌸"
            | "⌷"
            | "≡"
            | "≢"
            | "⊆"
            | "⊃"
            | "∩"
            | "∪"
            | "⍪"
            | "⍙"
            | "⍠"
    )
}
fn is_whitespace(s: &str) -> bool {
    // TODO: other weird forms of whitespace
    s.len() == 1 && s.bytes().nth(0).unwrap().is_ascii_whitespace()
}
fn is_digit(s: &str) -> bool {
    "0123456789".contains(s)
}
fn is_identifier(s: &str) -> bool {
    !is_whitespace(s) && !is_builtin(s)
}

/// token kind
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Ident,
    // any punctuation or APL character
    Builtin,
    Int(i64),
    Float(f64),
    Complex(f64, f64),
    String,
    EOF,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Span {
    line: usize,
    grapheme: usize,
}

/// token
#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    /// kind
    kind: TokenKind,
    /// lexeme
    lexeme: String,
}

impl Token {
    /// new
    pub(crate) fn new(kind: TokenKind, lexeme: impl ToString) -> Token {
        Token {
            kind,
            lexeme: lexeme.to_string(),
        }
    }
}

const YES_EXTENDED_GRAPHEMES: bool = true;

/// scanner
pub struct Scanner<'a> {
    graphemes: Peekable<Graphemes<'a>>,
    source: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            graphemes: source.graphemes(YES_EXTENDED_GRAPHEMES).peekable(),
            source,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, SponkError> {
        let mut grapheme = self.graphemes.next().unwrap_or("");
        while is_whitespace(grapheme) {
            grapheme = self.graphemes.next().unwrap_or("");
        }

        match grapheme {
            "" => Ok(Token::new(TokenKind::EOF, "")),
            g @ "(" => Ok(Token::new(TokenKind::LeftParen, g)),
            g @ ")" => Ok(Token::new(TokenKind::RightParen, g)),
            g @ "{" => Ok(Token::new(TokenKind::LeftBrace, g)),
            g @ "}" => Ok(Token::new(TokenKind::RightBrace, g)),
            g @ "[" => Ok(Token::new(TokenKind::LeftBracket, g)),
            g @ "]" => Ok(Token::new(TokenKind::RightBracket, g)),
            g @ "¯" => self.number(g),
            g @ "'" => self.string(g),
            g if is_builtin(g) => Ok(Token::new(TokenKind::Builtin, g)),
            g if is_digit(g) => self.number(g),
            g if is_whitespace(g) => unreachable!(),
            g => self.ident(g),
        }
    }

    fn string(&mut self, grapheme: &str) -> Result<Token, SponkError> {
        let mut string = String::from(grapheme);
        while let Some(&grapheme) = self.graphemes.peek() {
            if grapheme == "\\" {
                self.graphemes.next().unwrap();
                if let Some(&quote) = self.graphemes.peek() {
                    if quote != "'" {
                        return Err(SponkError::UnknownEscapeCode);
                    }
                }
                string.push_str("\\'");
            } else {
                string.push_str(grapheme);
            }

            if grapheme == "'" {
                self.graphemes.next().unwrap();
                return Ok(Token::new(TokenKind::String, string));
            }

            self.graphemes.next().unwrap();
        }

        Err(SponkError::UnterminatedString)
    }

    fn ident(&mut self, grapheme: &str) -> Result<Token, SponkError> {
        let mut ident = String::from(grapheme);
        while let Some(&grapheme) = self.graphemes.peek() {
            if is_identifier(grapheme) {
                ident.push_str(grapheme);
            } else {
                break;
            }
            self.graphemes.next().unwrap();
        }
        Ok(Token::new(TokenKind::Ident, ident))
    }

    fn number(&mut self, grapheme: &str) -> Result<Token, SponkError> {
        let mut number = String::from(grapheme);

        while let Some(&grapheme) = self.graphemes.peek() {
            if is_digit(grapheme) {
                number.push_str(grapheme);
            } else if grapheme == "." {
                return self.float(number);
            } else if grapheme == "J" || grapheme == "j" {
                return self.complex(number);
            } else if grapheme == "E" || grapheme == "e" {
                return self.scientific(number);
            } else {
                break;
            }
            self.graphemes.next().unwrap();
        }

        Ok(Token::new(
            TokenKind::Int(number.replace("¯", "-").parse()?),
            number,
        ))
    }

    fn float(&mut self, mut number: String) -> Result<Token, SponkError> {
        assert_eq!(".", self.graphemes.next().unwrap());
        number.push('.');

        while let Some(&grapheme) = self.graphemes.peek() {
            if is_digit(grapheme) {
                number.push_str(grapheme);
            } else if grapheme == "J" || grapheme == "j" {
                return self.complex_float(number);
            } else if grapheme == "E" || grapheme == "e" {
                return self.scientific_float(number);
            } else {
                break;
            }
            self.graphemes.next().unwrap();
        }

        Ok(Token::new(
            TokenKind::Float(number.replace("¯", "-").parse()?),
            number,
        ))
    }

    fn complex(&mut self, _number: String) -> Result<Token, SponkError> {
        todo!("complex integer")
    }

    fn scientific(&mut self, _number: String) -> Result<Token, SponkError> {
        todo!("scientific integer")
    }

    fn complex_float(&mut self, _number: String) -> Result<Token, SponkError> {
        todo!("complex float")
    }

    fn scientific_float(&mut self, _number: String) -> Result<Token, SponkError> {
        todo!("scientific float")
    }

    fn complex_scientific_float(&mut self, _number: String) -> Result<Token, SponkError> {
        todo!("complex scientific float")
    }
}

impl Iterator for Scanner<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token().ok().filter(|t| t.kind != TokenKind::EOF)
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn scan1() {
        let mut s = Scanner::new("{⍎:EJ≠<≠∨jjjjjj");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "⍎")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "EJ")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "≠")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "<")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "≠")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "∨")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "jjjjjj")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan2() {
        let mut s = Scanner::new("{} :{}]:][:{]:}}{]:");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBrace, "}")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBrace, "}")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBracket, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBracket, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBracket, "[")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBracket, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBrace, "}")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBrace, "}")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBracket, "]")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
    }

    #[test]
    fn scan3() {
        let mut s = Scanner::new("                 1");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Int(1), "1")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan4() {
        let mut s = Scanner::new("{223}");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Int(223), "223")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::RightBrace, "}")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan5() {
        let mut s = Scanner::new("⍵ ⍵:3.14:{  ");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "⍵")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "⍵")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
        assert_eq!(
            s.next_token(),
            Ok(Token::new(TokenKind::Float(3.14), "3.14"))
        );
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan6() {
        let mut s = Scanner::new("i:ii i 32.4:i");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "i")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "ii")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "i")));
        assert_eq!(
            s.next_token(),
            Ok(Token::new(TokenKind::Float(32.4), "32.4"))
        );
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
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
        let mut s = Scanner::new("{i3289⍺jeiwe328 38.3");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::LeftBrace, "{")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "i3289")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "⍺")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "jeiwe328")));
        assert_eq!(
            s.next_token(),
            Ok(Token::new(TokenKind::Float(38.3), "38.3"))
        );
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan10() {
        let mut s = Scanner::new("amp:=⍺:⍺⍵:⍺:⍵");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Ident, "amp")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "=")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "⍺")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "⍺")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "⍵")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "⍺")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, ":")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "⍵")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::EOF, "")));
    }

    #[test]
    fn scan11() {
        let v: Vec<_> = Scanner::new("x ← 1 2 3 4 5").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["x", "←", "1", "2", "3", "4", "5"]);
        let v: Vec<_> = Scanner::new("y ← 6 7 8 9 10").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["y", "←", "6", "7", "8", "9", "10"]);
        let v: Vec<_> = Scanner::new("x + y").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["x", "+", "y"]);
        let v: Vec<_> = Scanner::new("⍴⍴x").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["⍴", "⍴", "x"]);
        let v: Vec<_> = Scanner::new("{⍵+⍵}x").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["{", "⍵", "+", "⍵", "}", "x"]);
        let v: Vec<_> = Scanner::new("{1+⍵}(f 1 2 3 4 5)")
            .map(|t| t.lexeme)
            .collect();
        assert_eq!(
            v,
            vec!["{", "1", "+", "⍵", "}", "(", "f", "1", "2", "3", "4", "5", ")"]
        );
        let v: Vec<_> = Scanner::new("amp:=[:[ ]: [:]").map(|t| t.lexeme).collect();
        assert_eq!(
            v,
            vec!["amp", ":", "=", "[", ":", "[", "]", ":", "[", ":", "]"]
        );
    }

    #[test]
    fn scan12() {
        let v: Vec<_> = Scanner::new("+...+~$#@*-*::").map(|t| t.lexeme).collect();
        assert_eq!(
            v,
            vec!["+", ".", ".", ".", "+", "~", "$", "#", "@", "*", "-", "*", ":", ":"]
        );
    }

    #[test]
    fn scan13() {
        let mut s = Scanner::new("!⌹÷÷×⍥");
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "!")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "⌹")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "÷")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "÷")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "×")));
        assert_eq!(s.next_token(), Ok(Token::new(TokenKind::Builtin, "⍥")));
    }

    #[test]
    fn scan14() {
        let v: Vec<_> = Scanner::new("x+y").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["x", "+", "y"]);
    }

    #[test]
    fn scan15() {
        let mut s = Scanner::new("¯1398 heeehe");
        assert_eq!(s.next(), Some(Token::new(TokenKind::Int(-1398), "¯1398")));
        assert_eq!(s.next(), Some(Token::new(TokenKind::Ident, "heeehe")));
    }

    #[test]
    fn scan16() {
        let mut s = Scanner::new("¯13.98 heeehe");
        assert_eq!(
            s.next(),
            Some(Token::new(TokenKind::Float(-13.98), "¯13.98"))
        );
        assert_eq!(s.next(), Some(Token::new(TokenKind::Ident, "heeehe")));
    }

    #[should_panic]
    #[test]
    fn scan17() {
        let mut s = Scanner::new("¯¯3");
        panic!("{:?}", s.next_token().unwrap());
    }
}
