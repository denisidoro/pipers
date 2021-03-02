#[macro_use]
extern crate lazy_static;

mod metadata;
mod process;
mod sanitize;
mod token;

pub use process::convert;
