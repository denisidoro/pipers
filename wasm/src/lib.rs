use wasm_bindgen::prelude::*;

#[macro_use]
extern crate lazy_static;

mod metadata;
mod process;
mod sanitize;
mod token;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn convert(txt: &str) -> String {
    process::convert(txt)
}
