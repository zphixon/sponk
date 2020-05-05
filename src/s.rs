use super::*;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum Tk {
  La, Ra, Lb, Rb, // [ ] { }
  Q, Qi, Ql, Qr, // :{ :i :[ :]
  Uq, Uqi, Uqla, Uqra, // }: i: [: ]:
  Aw, Av, Aav, // =. =: :=:
  L,
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
    self.b = self.c;
    if self.e() { N } else {
      self.sws();
      S(T::n(match self.a() {
        b'[' if self.p() == b':' => { self.a(); Tk::Uqla },
        b'[' => Tk::La,

        b']' if self.p() == b':' => { self.a(); Tk::Uqra },
        b']' => Tk::Ra,

        b'{' => Tk::Lb,

        b'}' if self.p() == b':' => { self.a(); Tk::Uq },
        b'}' => Tk::Rb,

        b':' => match self.p() {
          b'[' => { self.a(); Tk::Ql },
          b']' => { self.a(); Tk::Qr },
          b'{' => { self.a(); Tk::Q },
          _ => todo!("Tk::Qi")
        }
        _ => todo!("Tk::Uqi, Tk::I")
      }, self.l()))
    }
  }

  fn sws(&mut self) {
    while self.p().is_ascii_whitespace() { self.a(); }
  }

  fn e(&self) -> bool {
    self.c >= self.s.len()
  }

  fn a(&mut self) -> u8 {
    self.c += 1;
    self.s[self.c-1]
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
    let mut s = Sc::n("[[:]:]");
    assert_eq!(s.nt(), S(T::n(Tk::La, "[")));
    assert_eq!(s.nt(), S(T::n(Tk::Uqla, "[:")));
    assert_eq!(s.nt(), S(T::n(Tk::Uqra, "]:")));
    assert_eq!(s.nt(), S(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), N);
    assert_eq!(s.nt(), N);
    assert_eq!(s.nt(), N);
    assert_eq!(s.nt(), N);
    assert_eq!(s.nt(), N);
  }

  #[test]
  fn s2() {
    let mut s = Sc::n("{}:{}]:][:{]:}}{]:");
    assert_eq!(s.nt(), S(T::n(Tk::Lb, "{")));
    assert_eq!(s.nt(), S(T::n(Tk::Uq, "}:")));
    assert_eq!(s.nt(), S(T::n(Tk::Lb, "{")));
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
  }
}
