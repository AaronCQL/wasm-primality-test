mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct PrimeResult(pub bool, pub u64);

#[wasm_bindgen]
pub fn is_definite_prime(n: u64) -> PrimeResult {
    if n <= 3 {
        return PrimeResult(n > 1, 0);
    }
    if n % 2 == 0 {
        return PrimeResult(false, 2);
    }
    if n % 3 == 0 {
        return PrimeResult(false, 3);
    }

    let mut i: u64 = 5;
    while i.pow(2) <= n {
        if n % i == 0 {
            return PrimeResult(false, i);
        }
        if n % (i + 2) == 0 {
            return PrimeResult(false, i + 2);
        }
        i += 6;
    }
    PrimeResult(true, 0)
}
