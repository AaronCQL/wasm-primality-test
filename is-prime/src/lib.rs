mod utils;

use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_integer::Integer;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn is_probably_prime(n: &str, k: u32) -> bool {
    let n = n.parse::<BigUint>().unwrap();
    let one: BigUint = 1.to_biguint().unwrap();
    let two: BigUint = 2.to_biguint().unwrap();
    let three: BigUint = 3.to_biguint().unwrap();
    let four: BigUint = 4.to_biguint().unwrap();

    if n <= one || n == four {
        return false;
    }
    if n <= three {
        return true;
    }

    let mut d = &n - &one;
    while d.is_even() {
        d /= &two;
    }

    for _ in 0..=k {
        if miller_test(&mut d, &n) == false {
            return false;
        }
    }

    true
}

fn miller_test(d: &mut BigUint, n: &BigUint) -> bool {
    let one: BigUint = 1.to_biguint().unwrap();
    let two: BigUint = 2.to_biguint().unwrap();
    let mut rng = rand::thread_rng();

    // pick random number a in [2..n-2]
    let a = rng.gen_biguint_range(&two, &(n - &one));

    // compute a^d % n
    let mut x = a.modpow(d, n);

    if x == one || x == (n - &one) {
        return true;
    }

    while *d != (n - &one) {
        x = x.modpow(&two, n);
        *d *= &two;

        if x == one {
            return false;
        }
        if x == (n - &one) {
            return true;
        }
    }

    false
}

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
