#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate anyhow;
extern crate thiserror;
extern crate unicode_segmentation;

mod array;
mod error;
mod parser;
mod scanner;

pub mod prelude {
    pub use crate::anyhow::{anyhow, Context, Error, Result};
    pub use crate::array::{Array, Element};
    pub use crate::error::ErrorKind;
    pub use crate::parser::{parse, Expression, Statement};
    pub use crate::scanner::{Scanner, Span, Token, TokenKind};
}
