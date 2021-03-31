#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod array;
mod error;
mod parser;
mod scanner;

pub mod prelude {
  pub use array::{Array, Element};
  pub use error::SponkError;
  pub use parser::{parse, Statement, Expression};
  pub use scanner::{Scanner, Token, TokenKind};
}
