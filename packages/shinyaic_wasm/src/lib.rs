extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::paint::paint::paint as a;
use crate::render_tree as ff;

mod paint;
mod render_tree;

#[wasm_bindgen]
pub fn greet(name: &str) {
    a();
}
