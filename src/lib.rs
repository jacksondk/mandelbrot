// fn main() {
//     println!("Hello, world!");
//     println!("Two lines");
// }


mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn greet() -> i32 {
    42
}


#[wasm_bindgen]
pub fn compute_row(imag: f64, min_real: f64, max_real: f64, steps: u16) -> Vec<u16> {
    let mut row = Vec::with_capacity(steps as usize);
    let step_size = (max_real - min_real) / (steps as f64);

    for i in 0..steps {
        let _real = min_real + (i as f64) * step_size;
        // Compute the value for each element here
        let value = i as u16; // Example computation
        row.push(value);
    }

    row
}
