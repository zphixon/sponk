//! scanner

use crate::prelude::{anyhow, ErrorKind, Result};

use unicode_segmentation::{Graphemes, UnicodeSegmentation};

/// Kinds of tokens
///
/// Tokens that must require symmetry have both forms as token types, the rest fall under Builtin or literal token types.
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

/// Indices into a language source
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Span {
    line: usize,
    grapheme_index_in_line: usize,
}

impl Span {
    /// Create a new span.
    pub(crate) fn new(line: usize, grapheme_index_in_line: usize) -> Span {
        Span {
            line,
            grapheme_index_in_line,
        }
    }

    /// Get the line of the span.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Get grapheme index in the line of the span.
    pub fn grapheme_index_in_line(&self) -> usize {
        self.grapheme_index_in_line
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "line {} char {}", self.line, self.grapheme_index_in_line)
    }
}

/// Tokens with kind, lexeme, and span information
#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    span: Span,
}

impl Token {
    /// Create a new token.
    pub(crate) fn new(kind: TokenKind, lexeme: impl ToString, span: Span) -> Token {
        Token {
            kind,
            lexeme: lexeme.to_string(),
            span,
        }
    }

    /// Create a token without a span. Only used in this file.
    pub fn no_span(kind: TokenKind, lexeme: impl ToString) -> Token {
        Token {
            kind,
            lexeme: lexeme.to_string(),
            span: Span {
                line: 0,
                grapheme_index_in_line: 0,
            },
        }
    }

    /// Compare a token with another, ignoring span information. Only used in this file.
    pub fn compare_no_span(&self, other: Token) -> bool {
        self.kind == other.kind && self.lexeme == other.lexeme
    }

    /// Get the kind of the token.
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    /// Get the lexeme of the token.
    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    /// Get the span of the token.
    pub fn span(&self) -> Span {
        self.span
    }
}

// We want to consider extended grapheme clusters, so we pass true to source.graphemes.
// Using a const to avoid magic numbers.
const YES_EXTENDED_GRAPHEMES: bool = true;

/// A scanner of Sponk language sources
pub struct Scanner<'a> {
    graphemes: std::iter::Peekable<Graphemes<'a>>,
    source: &'a str,
    line: usize,
    grapheme_index_in_line: usize,
}

impl<'a> Scanner<'a> {
    /// Create a new scanner from a source.
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            graphemes: source.graphemes(YES_EXTENDED_GRAPHEMES).peekable(),
            source,
            line: 1,
            grapheme_index_in_line: 0,
        }
    }

    // Advance to the next line, only called in next_grapheme.
    fn newline(&mut self) {
        self.line += 1;
        self.grapheme_index_in_line = 0;
    }

    // Get the current span of the scanner.
    fn span(&self) -> Span {
        Span::new(self.line, self.grapheme_index_in_line)
    }

    // Get the next grapheme from the grapheme iterator.
    fn next_grapheme(&mut self) -> Option<&'a str> {
        self.graphemes.next().map(|grapheme| {
            // increment the line number if necessary
            if grapheme == "\n" {
                self.newline();
            } else {
                self.grapheme_index_in_line += 1;
            }

            grapheme
        })
    }

    // Peek the next grapheme.
    fn peek_grapheme(&mut self) -> Option<&'a str> {
        self.graphemes.peek().cloned()
    }

    /// Get the next token from the source.
    pub fn next_token(&mut self) -> Result<Token> {
        // get the next grapheme
        let mut grapheme = self.next_grapheme().unwrap_or("");
        while util::is_whitespace(grapheme) {
            // skip past whitespace
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
            g @ "¯" => self.number(g), // negative number literals use an over-score
            g @ "'" => self.string(g),
            g if util::is_builtin(g) => Ok(Token::new(TokenKind::Builtin, g, self.span())),
            g if util::is_digit(g) => self.number(g),
            g if util::is_whitespace(g) => unreachable!(),
            g => self.ident(g),
        }
    }

    // Scan a string.
    fn string(&mut self, grapheme: &str) -> Result<Token> {
        // start creating a string
        let mut string = String::from(grapheme);

        // get the next grapheme
        // we will early return instead of exiting the loop normally
        while let Some(grapheme) = self.peek_grapheme() {
            // if we hit a backslash, it's an escape sequence
            if grapheme == "\\" {
                self.next_grapheme().unwrap();
                // we only have \' for now, if we had more later use a match statement
                if let Some(quote) = self.peek_grapheme() {
                    if quote != "'" {
                        return Err(anyhow!(ErrorKind::UnknownEscapeCode {
                            code: quote.to_string(),
                        }));
                    }
                }
                // add the escaped sequence to the string (\' only right now)
                string.push_str("\\'");
            } else {
                // add it to the string
                string.push_str(grapheme);
            }

            // when we've reached the end of the string
            if grapheme == "'" {
                self.next_grapheme().unwrap();
                return Ok(Token::new(TokenKind::String, string, self.span()));
            }

            // continue
            self.next_grapheme().unwrap();
        }

        // if we don't have any more tokens and we've made it out of the loop, the string is unterminated
        Err(anyhow!(ErrorKind::UnterminatedString { span: self.span() }))
    }

    // Scan an identifier.
    fn ident(&mut self, grapheme: &str) -> Result<Token> {
        // start the identifier
        let mut ident = String::from(grapheme);

        // while we have graphemes left
        while let Some(grapheme) = self.peek_grapheme() {
            // if the grapheme is ok to put in an identifier (ascii alphanum only currently)
            if util::is_identifier(grapheme) {
                // add it to the identifier
                ident.push_str(grapheme);
            } else {
                break;
            }
            self.next_grapheme().unwrap();
        }

        Ok(Token::new(TokenKind::Ident, ident, self.span()))
    }

    // Scan a number.
    fn number(&mut self, grapheme: &str) -> Result<Token> {
        // start the identifier
        let mut number = String::from(grapheme);

        // while we still have graphemes
        while let Some(grapheme) = self.peek_grapheme() {
            // if it's a digit, add it to the number
            if util::is_digit(grapheme) {
                number.push_str(grapheme);
            } else if grapheme == "." {
                // if we find a . we're scanning a float, continue doing that
                return self.float(number);
            } else if grapheme == "J" || grapheme == "j" {
                // if we find a j it's a complex number
                return self.complex(number);
            } else if grapheme == "E" || grapheme == "e" {
                // e is scientific notation
                return self.scientific(number);
            } else {
                break;
            }
            self.next_grapheme().unwrap();
        }

        Ok(Token::new(
            TokenKind::Int(number.replace("¯", "-").parse::<i64>().map_err(|e| {
                ErrorKind::SyntaxError {
                    // TODO there's probably a better way of doing this
                    why: anyhow!(e),
                    span: self.span(),
                }
            })?),
            number,
            self.span(),
        ))
    }

    // Scan a floating-point number.
    fn float(&mut self, mut number: String) -> Result<Token> {
        // sanity
        assert_eq!(".", self.next_grapheme().unwrap());
        number.push('.');

        // get the rest of the number, jumping out if it's not a simple float
        while let Some(grapheme) = self.peek_grapheme() {
            if util::is_digit(grapheme) {
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
            TokenKind::Float(number.replace("¯", "-").parse::<f64>().map_err(|e| {
                ErrorKind::SyntaxError {
                    why: anyhow!(e),
                    span: self.span(),
                }
            })?),
            number,
            self.span(),
        ))
    }

    fn complex(&mut self, _number: String) -> Result<Token> {
        todo!("complex integer")
    }

    fn scientific(&mut self, _number: String) -> Result<Token> {
        todo!("scientific integer")
    }

    fn complex_float(&mut self, _number: String) -> Result<Token> {
        todo!("complex float")
    }

    fn scientific_float(&mut self, _number: String) -> Result<Token> {
        todo!("scientific float")
    }

    fn complex_scientific_float(&mut self, _number: String) -> Result<Token> {
        todo!("complex scientific float")
    }
}

impl Iterator for Scanner<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token().ok().filter(|t| t.kind != TokenKind::EOF)
    }
}

mod util {
    /// Matches all symbols, minus single quote and one other that I don't remember lol
    pub(crate) fn is_builtin(s: &str) -> bool {
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

    pub(crate) fn is_whitespace(s: &str) -> bool {
        // TODO: other weird forms of whitespace
        s.len() == 1 && s.bytes().nth(0).unwrap().is_ascii_whitespace()
    }

    pub(crate) fn is_digit(s: &str) -> bool {
        "0123456789".contains(s)
    }

    /// Grapheme can be in an identifier if it's not ascii whitespace or builtin
    pub(crate) fn is_identifier(s: &str) -> bool {
        !is_whitespace(s) && !is_builtin(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[should_panic]
    #[test]
    fn scan19() {
        let mut s = Scanner::new(
            "578419057648432954637895647381946573821964378912467389216473812964739821",
        );
        println!("{:?}", s.next_token().unwrap());
    }
}
