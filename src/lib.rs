#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

type O<t> = Option<t>;
type R<t> = Result<t, E>;
type U = usize;

use std::option::Option::Some as S;
use std::option::Option::None as N;

mod a;
mod e;
mod p;
mod s;

use a::*;
use e::*;
use p::*;
use s::*;
