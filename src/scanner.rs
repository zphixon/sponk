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
    grapheme_index_in_line: usize,
}

impl Span {
    pub fn new(line: usize, grapheme_index_in_line: usize) -> Span {
        Span {
            line,
            grapheme_index_in_line,
        }
    }
}

/// token
#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    /// kind
    kind: TokenKind,
    /// lexeme
    lexeme: String,
    span: Span,
}

impl Token {
    /// new
    pub(crate) fn new(kind: TokenKind, lexeme: impl ToString, span: Span) -> Token {
        Token {
            kind,
            lexeme: lexeme.to_string(),
            span,
        }
    }

    pub(crate) fn no_span(kind: TokenKind, lexeme: impl ToString) -> Token {
        Token {
            kind,
            lexeme: lexeme.to_string(),
            span: Span {
                line: 0,
                grapheme_index_in_line: 0,
            },
        }
    }

    pub(crate) fn compare_no_span(&self, other: Token) -> bool {
        self.kind == other.kind && self.lexeme == other.lexeme
    }
}

const YES_EXTENDED_GRAPHEMES: bool = true;

/// scanner
pub struct Scanner<'a> {
    graphemes: Peekable<Graphemes<'a>>,
    source: &'a str,
    line: usize,
    grapheme_index_in_line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            graphemes: source.graphemes(YES_EXTENDED_GRAPHEMES).peekable(),
            source,
            line: 1,
            grapheme_index_in_line: 0,
        }
    }

    fn newline(&mut self) {
        self.line += 1;
        self.grapheme_index_in_line = 0;
    }

    fn span(&self) -> Span {
        Span::new(self.line, self.grapheme_index_in_line)
    }

    fn next_grapheme(&mut self) -> Option<&'a str> {
        self.graphemes.next().map(|grapheme| {
            if grapheme == "\n" {
                self.newline();
            } else {
                self.grapheme_index_in_line += 1;
            }

            grapheme
        })
    }

    fn peek_grapheme(&mut self) -> Option<&'a str> {
        self.graphemes.peek().cloned()
    }

    pub fn next_token(&mut self) -> Result<Token, SponkError> {
        let mut grapheme = self.next_grapheme().unwrap_or("");
        while is_whitespace(grapheme) {
            grapheme = self.next_grapheme().unwrap_or("");
        }

        match grapheme {
            "" => Ok(Token::new(TokenKind::EOF, "", self.span())),
            g @ "(" => Ok(Token::new(TokenKind::LeftParen, g, self.span())),
            g @ ")" => Ok(Token::new(TokenKind::RightParen, g, self.span())),
            g @ "{" => Ok(Token::new(TokenKind::LeftBrace, g, self.span())),
            g @ "}" => Ok(Token::new(TokenKind::RightBrace, g, self.span())),
            g @ "[" => Ok(Token::new(TokenKind::LeftBracket, g, self.span())),
            g @ "]" => Ok(Token::new(TokenKind::RightBracket, g, self.span())),
            g @ "¯" => self.number(g),
            g @ "'" => self.string(g),
            g if is_builtin(g) => Ok(Token::new(TokenKind::Builtin, g, self.span())),
            g if is_digit(g) => self.number(g),
            g if is_whitespace(g) => unreachable!(),
            g => self.ident(g),
        }
    }

    fn string(&mut self, grapheme: &str) -> Result<Token, SponkError> {
        let mut string = String::from(grapheme);
        while let Some(grapheme) = self.peek_grapheme() {
            if grapheme == "\\" {
                self.next_grapheme().unwrap();
                if let Some(quote) = self.peek_grapheme() {
                    if quote != "'" {
                        return Err(SponkError::UnknownEscapeCode);
                    }
                }
                string.push_str("\\'");
            } else {
                string.push_str(grapheme);
            }

            if grapheme == "'" {
                self.next_grapheme().unwrap();
                return Ok(Token::new(TokenKind::String, string, self.span()));
            }

            self.next_grapheme().unwrap();
        }

        Err(SponkError::UnterminatedString)
    }

    fn ident(&mut self, grapheme: &str) -> Result<Token, SponkError> {
        let mut ident = String::from(grapheme);
        while let Some(grapheme) = self.peek_grapheme() {
            if is_identifier(grapheme) {
                ident.push_str(grapheme);
            } else {
                break;
            }
            self.next_grapheme().unwrap();
        }
        Ok(Token::new(TokenKind::Ident, ident, self.span()))
    }

    fn number(&mut self, grapheme: &str) -> Result<Token, SponkError> {
        let mut number = String::from(grapheme);

        while let Some(grapheme) = self.peek_grapheme() {
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
            self.next_grapheme().unwrap();
        }

        Ok(Token::new(
            TokenKind::Int(number.replace("¯", "-").parse()?),
            number,
            self.span(),
        ))
    }

    fn float(&mut self, mut number: String) -> Result<Token, SponkError> {
        assert_eq!(".", self.next_grapheme().unwrap());
        number.push('.');

        while let Some(grapheme) = self.peek_grapheme() {
            if is_digit(grapheme) {
                number.push_str(grapheme);
            } else if grapheme == "J" || grapheme == "j" {
                return self.complex_float(number);
            } else if grapheme == "E" || grapheme == "e" {
                return self.scientific_float(number);
            } else {
                break;
            }
            self.next_grapheme().unwrap();
        }

        Ok(Token::new(
            TokenKind::Float(number.replace("¯", "-").parse()?),
            number,
            self.span(),
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
    use super::*;
    use crate::prelude::*;

    #[test]
    fn scan1() {
        let mut s = Scanner::new("{⍎:EJ≠<≠∨jjjjjj");
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::LeftBrace, "{")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "⍎")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Ident, "EJ")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "≠")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "<")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "≠")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "∨")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Ident, "jjjjjj")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::EOF, "")));
    }

    #[test]
    fn scan2() {
        let mut s = Scanner::new("{} :{}]:][:{]:}}{]:");
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::LeftBrace, "{")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::RightBrace, "}")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::LeftBrace, "{")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::RightBrace, "}")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::RightBracket, "]")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::RightBracket, "]")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::LeftBracket, "[")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::LeftBrace, "{")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::RightBracket, "]")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::RightBrace, "}")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::RightBrace, "}")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::LeftBrace, "{")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::RightBracket, "]")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
    }

    #[test]
    fn scan3() {
        let mut s = Scanner::new("                 1");
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Int(1), "1")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::EOF, "")));
    }

    #[test]
    fn scan4() {
        let mut s = Scanner::new("{223}");
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::LeftBrace, "{")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Int(223), "223")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::RightBrace, "}")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::EOF, "")));
    }

    #[test]
    fn scan5() {
        let mut s = Scanner::new("⍵ ⍵:3.14:{  ");
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "⍵")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "⍵")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Float(3.14), "3.14")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::LeftBrace, "{")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::EOF, "")));
    }

    #[test]
    fn scan6() {
        let mut s = Scanner::new("i:ii i 32.4:i");
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Ident, "i")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Ident, "ii")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Ident, "i")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Float(32.4), "32.4")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Ident, "i")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::EOF, "")));
    }

    #[test]
    fn scan7() {
        let mut s = Scanner::new("  'hello \\' world'");
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::String, "'hello \\' world'")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::EOF, "")));
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
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::LeftBrace, "{")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Ident, "i3289")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "⍺")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Ident, "jeiwe328")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Float(38.3), "38.3")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::EOF, "")));
    }

    #[test]
    fn scan10() {
        let mut s = Scanner::new("amp:=⍺:⍺⍵:⍺:⍵");
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Ident, "amp")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "=")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "⍺")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "⍺")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "⍵")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "⍺")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, ":")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "⍵")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::EOF, "")));
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
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "!")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "⌹")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "÷")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "÷")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "×")));
        assert!(s
            .next_token()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Builtin, "⍥")));
    }

    #[test]
    fn scan14() {
        let v: Vec<_> = Scanner::new("x+y").map(|t| t.lexeme).collect();
        assert_eq!(v, vec!["x", "+", "y"]);
    }

    #[test]
    fn scan15() {
        let mut s = Scanner::new("¯1398 heeehe");
        assert!(s
            .next()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Int(-1398), "¯1398")));
        assert!(s
            .next()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Ident, "heeehe")));
    }

    #[test]
    fn scan16() {
        let mut s = Scanner::new("¯13.98 heeehe");
        assert!(s
            .next()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Float(-13.98), "¯13.98")));
        assert!(s
            .next()
            .unwrap()
            .compare_no_span(Token::no_span(TokenKind::Ident, "heeehe")));
    }

    #[should_panic]
    #[test]
    fn scan17() {
        let mut s = Scanner::new("¯¯3");
        println!("{:?}", s.next_token().unwrap());
    }

    #[should_panic]
    #[test]
    fn scan18() {
        let mut s = Scanner::new("'");
        println!("{:?}", s.next_token().unwrap());
    }
}
