mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn rust_string() -> String {
    "rust 🦀".into()
}

#[wasm_bindgen]
pub fn add_one(i: i32) -> i32 {
    i + 1
}
