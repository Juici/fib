use std::collections::HashMap;

use num_bigint::BigUint;
use num_traits::{One, Zero};

use cached::proc_macro::cached;

/// Computes the value of the `n`th term of the Fibonacci sequence.
#[inline]
pub fn fib(n: u128) -> BigUint {
    _fib(n)
}

#[cached(type = "HashMap<u128, BigUint>", create = "{ HashMap::with_capacity(100) }")]
fn _fib(n: u128) -> BigUint {
    match n {
        0 => return BigUint::zero(),
        1 | 2 => return BigUint::one(),
        _ => {}
    }

    if n & 1 != 0 {
        let k = (n + 1) / 2;

        let mut f0 = _fib(k);
        let mut f1 = _fib(k - 1);

        let mut tmp = f0.clone();
        f0 *= &tmp;

        tmp.clone_from(&f1);
        f1 *= &tmp;

        f0 + f1
    } else {
        let k = n / 2;

        let f0 = _fib(k);
        let mut f1 = _fib(k - 1);

        f1 <<= 1u8;
        f1 += &f0;
        f1 *= &f0;

        f1
    }
}
