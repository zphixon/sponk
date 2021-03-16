//! scanner

use super::*;

/// token kind
#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum Tk {
  /// left paren
  Lp,
  /// right paren
  Rp,
  /// left arg
  La,
  /// right arg
  Ra,
  /// left brace
  Lb,
  /// right brace
  Rb,
  /// identifier
  I(U),
  /// colon equal
  Coeq,
  /// equal
  Eq,
  /// colon
  Co,
  /// number
  Dg,
  /// string
  Str,
  /// eof
  E,
}

/// token
#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) struct T<'a> {
  /// kind
  k: Tk,
  /// lexeme
  l: &'a str,
}

impl T<'_> {
  /// new
  pub(crate) fn n(k: Tk, l: &str) -> T<'_> {
    T { k, l }
  }
}

/// scanner
#[derive(Debug)]
pub(crate) struct Sc<'a> {
  /// beginning
  b: U,
  /// current
  c: U,
  /// source
  s: &'a [u8],
}

impl<'a> Sc<'a> {
  /// new
  pub(crate) fn n(s: &str) -> Sc<'_> {
    Sc {
      b: 0,
      c: 0,
      s: s.as_bytes()
    }
  }

  /// next token
  pub(crate) fn nt(&mut self) -> R<T<'a>> {
    if self.e() { Ok(T::n(Tk::E, "")) } else {
      self.sws();
      self.b = self.c;
      Ok(T::n(match self.a() {
        b'\0' => return Ok(T::n(Tk::E, "")),
        b'\'' => self.st(),

        b':' => match self.p() {
          b'=' => { self.a(); Ok(Tk::Coeq) }
          _ => Ok(Tk::Co),
        }

        b'=' => Ok(Tk::Eq),

        b'(' => Ok(Tk::Lp),
        b')' => Ok(Tk::Rp),

        b'[' => Ok(Tk::La),
        b']' => Ok(Tk::Ra),

        b'{' => Ok(Tk::Lb),
        b'}' => Ok(Tk::Rb),

        c if c.is_ascii_digit() => self.dg(),
        c if c.is_ascii_alphabetic() => Ok(self.id()),
        c if c.is_ascii_punctuation() => Ok(self.op()),
        c => { dbg!(self); dbg!(c); unreachable!() },
      }?, self.l()?))
    }
  }

  /// string
  fn st(&mut self) -> R<Tk> {
    while !self.e() && self.p() != b'\'' {
      if self.p() == b'\\' {
        self.a();
        if self.p() != b'\'' { return Err(E::Uec) }
      }
      self.a();
    }
    if self.e() { Err(E::Us) }
    else { self.a(); Ok(Tk::Str) }
  }

  /// builtin op
  fn op(&mut self) -> Tk {
    let mut v = 0;
    while self.p() == b'.' { self.a(); v += 1; }
    Tk::I(v)
  }

  /// identifier
  fn id(&mut self) -> Tk {
    while self.p().is_ascii_alphanumeric() { self.a(); }
    let mut v = 0;
    while self.p() == b'.' { self.a(); v += 1; }
    Tk::I(v)
  }

  /// number
  fn dg(&mut self) -> R<Tk> {
    while self.p().is_ascii_digit() { self.a(); }
    if self.p() == b'.' {
      self.a();
      while self.p().is_ascii_digit() { self.a(); }
      self.l()?.parse::<f64>()?;
    } else {
      self.l()?.parse::<i64>()?;
    }
    Ok(Tk::Dg)
  }

  /// slurp whitespace
  fn sws(&mut self) {
    while self.p().is_ascii_whitespace() { self.a(); }
  }

  /// at end
  fn e(&self) -> bool {
    self.c >= self.s.len()
  }

  /// advance
  fn a(&mut self) -> u8 {
    self.c += 1;
    self.s.get(self.c-1).copied().unwrap_or(b'\0')
  }

  /// peek
  fn p(&mut self) -> u8 {
    if self.e() { b'\0' } else { self.s[self.c] }
  }

  /// lexeme
  fn l(&self) -> R<&'a str> {
    Ok(std::str::from_utf8(&self.s[self.b..self.c])?)
  }
}

impl<'a> Iterator for Sc<'a> {
  type Item = T<'a>;
  fn next(&mut self) -> Option<Self::Item> {
    self.nt().ok().filter(|t| t.k != Tk::E)
  }
}

#[cfg(test)]
mod t {
  use super::*;
  #[test]
  fn s() {
    let mut s = Sc::n("[[::[]]::]");
    assert_eq!(s.nt(), Ok(T::n(Tk::La, "[")));
    assert_eq!(s.nt(), Ok(T::n(Tk::La, "[")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::La, "[")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), Ok(T::n(Tk::E, "")));
  }

  #[test]
  fn s2() {
    let mut s = Sc::n("{} :{}]:][:{]:}}{]:");
    assert_eq!(s.nt(), Ok(T::n(Tk::Lb, "{")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Rb, "}")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Lb, "{")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Rb, "}")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), Ok(T::n(Tk::La, "[")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Lb, "{")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Rb, "}")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Rb, "}")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Lb, "{")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
  }

  #[test]
  fn s3() {
    let mut s = Sc::n("                 1");
    assert_eq!(s.nt(), Ok(T::n(Tk::Dg, "1")));
    assert_eq!(s.nt(), Ok(T::n(Tk::E, "")));
  }

  #[test]
  fn s4() {
    let mut s = Sc::n("{223}");
    assert_eq!(s.nt(), Ok(T::n(Tk::Lb, "{")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Dg, "223")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Rb, "}")));
    assert_eq!(s.nt(), Ok(T::n(Tk::E, "")));
  }

  #[test]
  fn s5() {
    let mut s = Sc::n("] ]:3.14:{  ");
    assert_eq!(s.nt(), Ok(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Dg, "3.14")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Lb, "{")));
    assert_eq!(s.nt(), Ok(T::n(Tk::E, "")));
  }

  #[test]
  fn s6() {
    let mut s = Sc::n("i:ii i 32.4:i");
    assert_eq!(s.nt(), Ok(T::n(Tk::I(0), "i")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::I(0), "ii")));
    assert_eq!(s.nt(), Ok(T::n(Tk::I(0), "i")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Dg, "32.4")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::I(0), "i")));
    assert_eq!(s.nt(), Ok(T::n(Tk::E, "")));
  }

  #[test]
  fn s7() {
    let mut s = Sc::n("  'hello \\' world'");
    assert_eq!(s.nt(), Ok(T::n(Tk::Str, "'hello \\' world'")));
    assert_eq!(s.nt(), Ok(T::n(Tk::E, "")));
  }

  #[test]
  fn s8() {
    let mut s = Sc::n("]} [{ ]}[{'}:heiojewojoije' }{} {  }  {[} }   :] [: :]}[]['hello '][ ]:{[}[:]   }[:{]:}  ");
    let mut i = 0;
    while s.next().is_some() { i += 1; }
    assert_eq!(i, 46);
  }

  #[test]
  fn s9() {
    let mut s = Sc::n("{i3289:jeiwe328 38.3");
    assert_eq!(s.nt(), Ok(T::n(Tk::Lb, "{")));
    assert_eq!(s.nt(), Ok(T::n(Tk::I(0), "i3289")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::I(0), "jeiwe328")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Dg, "38.3")));
    assert_eq!(s.nt(), Ok(T::n(Tk::E, "")));
  }

  #[test]
  fn s10() {
    let mut s = Sc::n("amp:=[:[]:[:]");
    assert_eq!(s.nt(), Ok(T::n(Tk::I(0), "amp")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Coeq, ":=")));
    assert_eq!(s.nt(), Ok(T::n(Tk::La, "[")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::La, "[")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::La, "[")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Co, ":")));
    assert_eq!(s.nt(), Ok(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), Ok(T::n(Tk::E, "")));
  }

  #[test]
  fn s11() {
    let v: Vec<_> = Sc::n("x := 1 2 3 4 5").map(|t| t.l).collect();
    assert_eq!(v, vec!["x", ":=", "1", "2", "3", "4", "5"]);
    let v: Vec<_> = Sc::n("y := 6 7 8 9 10").map(|t| t.l).collect();
    assert_eq!(v, vec!["y", ":=", "6", "7", "8", "9", "10"]);
    let v: Vec<_> = Sc::n("x + y").map(|t| t.l).collect();
    assert_eq!(v, vec!["x", "+", "y"]);
    let v: Vec<_> = Sc::n("#$x").map(|t| t.l).collect();
    assert_eq!(v, vec!["#", "$", "x"]);
    let v: Vec<_> = Sc::n("{]+]}x").map(|t| t.l).collect();
    assert_eq!(v, vec!["{", "]", "+", "]", "}", "x"]);
    let v: Vec<_> = Sc::n("{1+]}(f 1 2 3 4 5)").map(|t| t.l).collect();
    assert_eq!(v, vec!["{", "1", "+", "]", "}", "(", "f", "1", "2", "3", "4", "5", ")"]);
    let v: Vec<_> = Sc::n("amp:=[:[ ]: [:]").map(|t| t.l).collect();
    assert_eq!(v, vec!["amp", ":=", "[", ":", "[", "]", ":", "[", ":", "]"]);
  }

  #[test]
  fn s12() {
    let v: Vec<_> = Sc::n("+...+~$#@*-*::").map(|t| t.l).collect();
    assert_eq!(v, vec!["+...", "+", "~", "$", "#", "@", "*", "-", "*", ":", ":"]);
  }

  #[test]
  fn s13() {
    let mut s = Sc::n("+.. -. x....");
    assert_eq!(s.nt(), Ok(T::n(Tk::I(2), "+..")));
    assert_eq!(s.nt(), Ok(T::n(Tk::I(1), "-.")));
    assert_eq!(s.nt(), Ok(T::n(Tk::I(4), "x....")));
  }
}
