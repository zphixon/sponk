use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum Tk {
  Lp, Rp, // ( )
  La, Ra, Lb, Rb, I, // [ ] { } i
  Q, Qi, Qla, Qra, // :{ :i :[ :]
  Uqla, Uqra, // [: ]:
  Aw, Av, Aav, // =. =: =::
  Dg, Str, // 3 'str'
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) struct T<'a> {
  k: Tk,
  l: &'a str,
}

impl T<'_> {
  pub(crate) fn n(k: Tk, l: &str) -> T<'_> {
    T { k, l }
  }
}

pub(crate) struct Sc<'a> {
  b: U,
  c: U,
  s: &'a [u8],
}

impl<'a> Sc<'a> {
  pub(crate) fn n(s: &str) -> Sc<'_> {
    Sc {
      b: 0,
      c: 0,
      s: s.as_bytes()
    }
  }

  pub(crate) fn nt(&mut self) -> O<T<'a>> {
    if self.e() { N } else {
      self.sws();
      self.b = self.c;
      S(T::n(match self.a()? {
        b'\'' => self.st(),

        b'(' => S(Tk::Lp),
        b')' => S(Tk::Rp),

        b'[' if self.p() == b':' => { self.a()?; S(Tk::Uqla) },
        b'[' => S(Tk::La),

        b']' if self.p() == b':' => { self.a()?; S(Tk::Uqra) },
        b']' => S(Tk::Ra),

        b'{' => S(Tk::Lb),

        b'}' => S(Tk::Rb),

        b':' => match self.p() {
          b'[' => { self.a()?; S(Tk::Qla) },
          b']' => { self.a()?; S(Tk::Qra) },
          b'{' => { self.a()?; S(Tk::Q) },
          c if c.is_ascii_alphanumeric() => self.qi(),
          _ => N,
        }

        b'=' => match self.p() { // TODO: word =:, verb =::, adverb =:::, even higher order?
          b':' => { self.a()?;
            match self.p() {
              b':' => { self.a()?; S(Tk::Aav) },
              _ => S(Tk::Av),
            }
          }
          b'.' => { self.a()?; S(Tk::Aw) },
          _ => S(Tk::I)
        }

        c if c.is_ascii_digit() => self.dg(),
        c if c.is_ascii_alphabetic() => self.id(),
        c if c.is_ascii_punctuation() => self.op(),
        _ => N,
      }?, self.l()))
    }
  }

  fn st(&mut self) -> O<Tk> {
    while !self.e() && self.p() != b'\'' {
      if self.p() == b'\\' {
        self.a()?;
        if self.p() != b'\'' { return N }
      }
      self.a()?;
    }
    if self.e() { N }
    else { self.a()?; S(Tk::Str) }
  }

  fn op(&mut self) -> O<Tk> {
    while self.p() == b'.' { self.a()?; }
    S(Tk::I)
  }

  fn qi(&mut self) -> O<Tk> {
    self.id().map(|k| if k == Tk::I { S(Tk::Qi) } else { N })?
  }

  fn id(&mut self) -> O<Tk> {
    while self.p().is_ascii_alphanumeric() || self.p() == b'.' { self.a()?; }
    Some(Tk::I)
  }

  fn dg(&mut self) -> O<Tk> {
    while self.p().is_ascii_digit() { self.a()?; }
    if self.p() == b'.' {
      self.a()?;
      while self.p().is_ascii_digit() { self.a()?; }
      self.l().parse::<f64>().unwrap();
    } else {
      self.l().parse::<i64>().unwrap();
    }
    S(Tk::Dg)
  }

  fn sws(&mut self) {
    while self.p().is_ascii_whitespace() { let _ = self.a(); }
  }

  fn e(&self) -> bool {
    self.c >= self.s.len()
  }

  fn a(&mut self) -> O<u8> {
    self.c += 1;
    self.s.get(self.c-1).map(|u| *u)
  }

  fn p(&mut self) -> u8 {
    if self.e() { b'\0' } else { self.s[self.c] }
  }

  fn l(&self) -> &'a str {
    std::str::from_utf8(&self.s[self.b..self.c]).unwrap()
  }
}

impl<'a> Iterator for Sc<'a> {
  type Item = T<'a>;
  fn next(&mut self) -> Option<Self::Item> {
    self.nt()
  }
}

#[cfg(test)]
mod t {
  use super::*;
  #[test]
  fn s() {
    let mut s = Sc::n("[[::[]]::]");
    assert_eq!(s.nt(), S(T::n(Tk::La, "[")));
    assert_eq!(s.nt(), S(T::n(Tk::Uqla, "[:")));
    assert_eq!(s.nt(), S(T::n(Tk::Qla, ":[")));
    assert_eq!(s.nt(), S(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), S(T::n(Tk::Uqra, "]:")));
    assert_eq!(s.nt(), S(T::n(Tk::Qra, ":]")));
    assert_eq!(s.nt(), N);
  }

  #[test]
  fn s2() {
    let mut s = Sc::n("{} :{}]:][:{]:}}{]:");
    assert_eq!(s.nt(), S(T::n(Tk::Lb, "{")));
    assert_eq!(s.nt(), S(T::n(Tk::Rb, "}")));
    assert_eq!(s.nt(), S(T::n(Tk::Q, ":{")));
    assert_eq!(s.nt(), S(T::n(Tk::Rb, "}")));
    assert_eq!(s.nt(), S(T::n(Tk::Uqra, "]:")));
    assert_eq!(s.nt(), S(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), S(T::n(Tk::Uqla, "[:")));
    assert_eq!(s.nt(), S(T::n(Tk::Lb, "{")));
    assert_eq!(s.nt(), S(T::n(Tk::Uqra, "]:")));
    assert_eq!(s.nt(), S(T::n(Tk::Rb, "}")));
    assert_eq!(s.nt(), S(T::n(Tk::Rb, "}")));
    assert_eq!(s.nt(), S(T::n(Tk::Lb, "{")));
    assert_eq!(s.nt(), S(T::n(Tk::Uqra, "]:")));
    assert_eq!(s.nt(), N);
  }

  #[test]
  fn s3() {
    let mut s = Sc::n("                 1");
    assert_eq!(s.nt(), S(T::n(Tk::Dg, "1")));
    assert_eq!(s.nt(), N);
  }

  #[test]
  fn s4() {
    let mut s = Sc::n("{223}");
    assert_eq!(s.nt(), S(T::n(Tk::Lb, "{")));
    assert_eq!(s.nt(), S(T::n(Tk::Dg, "223")));
    assert_eq!(s.nt(), S(T::n(Tk::Rb, "}")));
    assert_eq!(s.nt(), N);
  }

  #[test]
  fn s5() {
    let mut s = Sc::n("] ]:3.14:{  ");
    assert_eq!(s.nt(), S(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), S(T::n(Tk::Uqra, "]:")));
    assert_eq!(s.nt(), S(T::n(Tk::Dg, "3.14")));
    assert_eq!(s.nt(), S(T::n(Tk::Q, ":{")));
    assert_eq!(s.nt(), N);
  }

  #[test]
  fn s6() {
    let mut s = Sc::n("i:ii i 32.4:i");
    assert_eq!(s.nt(), S(T::n(Tk::I, "i")));
    assert_eq!(s.nt(), S(T::n(Tk::Qi, ":ii")));
    assert_eq!(s.nt(), S(T::n(Tk::I, "i")));
    assert_eq!(s.nt(), S(T::n(Tk::Dg, "32.4")));
    assert_eq!(s.nt(), S(T::n(Tk::Qi, ":i")));
    assert_eq!(s.nt(), N);
  }

  #[test]
  fn s7() {
    let mut s = Sc::n("  'hello \\' world'");
    assert_eq!(s.nt(), S(T::n(Tk::Str, "'hello \\' world'")));
    assert_eq!(s.nt(), N);
  }

  #[test]
  fn s8() {
    let mut s = Sc::n("]} [{ ]}[{'}:heiojewojoije' }{} {  }  {[} }   :] [: :]}[]['hello '][ ]:{[}[:]   }[:{]:}  ");
    let mut i = 0;
    while s.nt().is_some() { i += 1; }
    assert_eq!(i, 39);
  }

  #[test]
  fn s9() {
    let mut s = Sc::n("{i3289:jeiwe328 38.3");
    assert_eq!(s.nt(), S(T::n(Tk::Lb, "{")));
    assert_eq!(s.nt(), S(T::n(Tk::I, "i3289")));
    assert_eq!(s.nt(), S(T::n(Tk::Qi, ":jeiwe328")));
    assert_eq!(s.nt(), S(T::n(Tk::Dg, "38.3")));
  }

  #[test]
  fn s10() {
    let mut s = Sc::n("amp=::[:[]:[:]");
    assert_eq!(s.nt(), S(T::n(Tk::I, "amp")));
    assert_eq!(s.nt(), S(T::n(Tk::Aav, "=::")));
    assert_eq!(s.nt(), S(T::n(Tk::Uqla, "[:")));
    assert_eq!(s.nt(), S(T::n(Tk::La, "[")));
    assert_eq!(s.nt(), S(T::n(Tk::Uqra, "]:")));
    assert_eq!(s.nt(), S(T::n(Tk::Uqla, "[:")));
    assert_eq!(s.nt(), S(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), N);
  }

  #[test]
  fn s11() {
    let v: Vec<_> = Sc::n("x =. 1 2 3 4 5").map(|t| t.l).collect();
    assert_eq!(v, vec!["x", "=.", "1", "2", "3", "4", "5"]);
    let v: Vec<_> = Sc::n("y =. 6 7 8 9 10").map(|t| t.l).collect();
    assert_eq!(v, vec!["y", "=.", "6", "7", "8", "9", "10"]);
    let v: Vec<_> = Sc::n("x + y").map(|t| t.l).collect();
    assert_eq!(v, vec!["x", "+", "y"]);
    let v: Vec<_> = Sc::n("#$x").map(|t| t.l).collect();
    assert_eq!(v, vec!["#", "$", "x"]);
    let v: Vec<_> = Sc::n("{]+]}x").map(|t| t.l).collect();
    assert_eq!(v, vec!["{", "]", "+", "]", "}", "x"]);
    let v: Vec<_> = Sc::n("{1+]}(f 1 2 3 4 5)").map(|t| t.l).collect();
    assert_eq!(v, vec!["{", "1", "+", "]", "}", "(", "f", "1", "2", "3", "4", "5", ")"]);
    let v: Vec<_> = Sc::n("amp=::[:[ ]: [:]").map(|t| t.l).collect();
    assert_eq!(v, vec!["amp", "=::", "[:", "[", "]:", "[:", "]"]);
  }
}
