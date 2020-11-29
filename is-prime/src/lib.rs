mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn is_definite_prime(n: u64) -> bool {
    if n <= 3 {
        return n > 1;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let upper_limit = (n as f64).sqrt() as u64;
    let mut i: u64 = 5;
    while i.pow(2) <= upper_limit {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
