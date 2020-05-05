#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

type u = usize;
type o<t> = Option<t>;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) enum Tk {
  La, Lam, Lad, Ra, Ram, Rad,
  B, Bc,
  Qim, Qbm, Qid, Qbd,
  I, Im, Id,
  E, Em, Ed,
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

pub(crate) struct S<'a> {
  b: u,
  c: u,
  s: &'a [u8],
}

impl<'a> S<'a> {
  pub(crate) fn n(s: &str) -> S<'_> {
    S {
      b: 0,
      c: 0,
      s: s.as_bytes()
    }
  }

  pub(crate) fn nt(&mut self) -> o<T<'a>> {
    self.b = self.c;
    if self.e() { None } else {
      Some(T::n(match self.a() {
        b'[' if self.p() == b'.' => { self.a(); Tk::Lam }
        b'[' if self.p() == b':' => { self.a(); Tk::Lad }
        b'[' => Tk::La,
        b']' if self.p() == b'.' => { self.a(); Tk::Ram }
        b']' if self.p() == b':' => { self.a(); Tk::Rad }
        b']' => Tk::Ra,

        b'{' => Tk::B,
        b'}' => Tk::Bc,
        b'.' => match self.p() {
          b'{' => { self.a(); Tk::Qbm },
          _ => todo!("Tk::Qim")
        }
        b':' => match self.p() {
          b'{' => { self.a(); Tk::Qbd },
          _ => todo!("Tk::Qid")
        }
        c => panic!("s {}", c as char)
      }, self.l()))
    }
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
    let mut s = S::n("[[.].]");
    assert_eq!(s.nt(), Some(T::n(Tk::La, "[")));
    assert_eq!(s.nt(), Some(T::n(Tk::Lam, "[.")));
    assert_eq!(s.nt(), Some(T::n(Tk::Ram, "].")));
    assert_eq!(s.nt(), Some(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), None);
    assert_eq!(s.nt(), None);
    assert_eq!(s.nt(), None);
    assert_eq!(s.nt(), None);
    assert_eq!(s.nt(), None);
  }

  #[test]
  fn s2() {
    let mut s = S::n("{}.{}].][.{]:}}{].");
    assert_eq!(s.nt(), Some(T::n(Tk::B, "{")));
    assert_eq!(s.nt(), Some(T::n(Tk::Bc, "}")));
    assert_eq!(s.nt(), Some(T::n(Tk::Qbm, ".{")));
    assert_eq!(s.nt(), Some(T::n(Tk::Bc, "}")));
    assert_eq!(s.nt(), Some(T::n(Tk::Ram, "].")));
    assert_eq!(s.nt(), Some(T::n(Tk::Ra, "]")));
    assert_eq!(s.nt(), Some(T::n(Tk::Lam, "[.")));
    assert_eq!(s.nt(), Some(T::n(Tk::B, "{")));
    assert_eq!(s.nt(), Some(T::n(Tk::Rad, "]:")));
    assert_eq!(s.nt(), Some(T::n(Tk::Bc, "}")));
    assert_eq!(s.nt(), Some(T::n(Tk::Bc, "}")));
    assert_eq!(s.nt(), Some(T::n(Tk::B, "{")));
    assert_eq!(s.nt(), Some(T::n(Tk::Ram, "].")));
  }
}
