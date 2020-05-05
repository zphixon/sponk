use super::*;

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum Tk {
  La, Ra, Lb, Rb, I, // [ ] { } i
  Q, Qi, Qla, Qra, // :{ :i :[ :]
  Uq, Uqi, Uqla, Uqra, // }: i: [: ]:
  Aw, Av, Aav, // =. =: :=:
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

        b'[' if self.p() == b':' => { self.a()?; S(Tk::Uqla) },
        b'[' => S(Tk::La),

        b']' if self.p() == b':' => { self.a()?; S(Tk::Uqra) },
        b']' => S(Tk::Ra),

        b'{' => S(Tk::Lb),

        b'}' if self.p() == b':' => { self.a()?; S(Tk::Uq) },
        b'}' => S(Tk::Rb),

        b':' => match self.p() {
          b'[' => { self.a()?; S(Tk::Qla) },
          b']' => { self.a()?; S(Tk::Qra) },
          b'{' => { self.a()?; S(Tk::Q) },
          _ => self.qi(),
        }

        c if c.is_ascii_digit() => self.dg(),
        c if c.is_ascii_alphabetic() => self.id(),
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

  fn qi(&mut self) -> O<Tk> {
    while self.p().is_ascii_alphabetic() { self.a()?; }
    Some(Tk::Qi)
  }

  fn id(&mut self) -> O<Tk> {
    while self.p().is_ascii_alphabetic() { self.a()?; }
    if self.p() == b':' { self.a()?; Some(Tk::Uqi) }
    else { Some(Tk::I) }
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
    let mut s = Sc::n("i::ii i 32.4:i");
    assert_eq!(s.nt(), S(T::n(Tk::Uqi, "i:")));
    assert_eq!(s.nt(), S(T::n(Tk::Qi, ":ii")));
    assert_eq!(s.nt(), S(T::n(Tk::I, "i")));
    assert_eq!(s.nt(), S(T::n(Tk::Dg, "32.4")));
    assert_eq!(s.nt(), S(T::n(Tk::Qi, ":i")));
    assert_eq!(s.nt(), N);
  }

  #[test]
  fn s7() {
    // 'hello \' world'
    let mut s = Sc::n("  'hello \\' world'");
    assert_eq!(s.nt(), S(T::n(Tk::Str, "'hello \\' world'")));
    assert_eq!(s.nt(), N);
  }

  #[test]
  fn s8() {
    let mut s = Sc::n("]}: [{ ]}[{'}:heiojewojoije' }{} {  }  {[}: }   :] [: :]}[]['hello '][ ]:{[}[:]   }[:{]:}  ");
    let mut i = 0;
    while s.nt().is_some() { i += 1; }
    assert_eq!(i, 39);
  }
}
