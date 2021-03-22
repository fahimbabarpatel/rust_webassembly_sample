use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(n1: i32, n2: i32) -> String {
    return format!("It's coming from webassembly written in Rust - {} + {} = {}", n1, n2, n1 + n2)
}