extern crate wasm_bindgen;
extern crate console_error_panic_hook;
extern crate wee_alloc;

use wasm_bindgen::prelude::*;
use console_error_panic_hook::set_once as set_panic_hook;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
  set_panic_hook();
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}