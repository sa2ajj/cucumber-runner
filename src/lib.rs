#[macro_use]
extern crate lazy_static;
extern crate regex;

mod error;
mod parser;
mod tokenizer;
mod types;
mod utils;

use std::result;

pub use error::Error;
pub use types::*;
pub use parser::parse;

pub type Result<T> = result::Result<T, Error>;

pub use utils::find_features;
