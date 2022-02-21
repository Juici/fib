use std::mem;

use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Computes the value of the `n`th term of the Fibonacci sequence.
pub fn fibonacci(n: u128) -> BigUint {
    let (mut f0, mut f1) = match n {
        0 => return BigUint::zero(),
        1 | 2 => return BigUint::one(),
        _ => (BigUint::zero(), BigUint::one()),
    };

    for _ in 0..n {
        let f2 = f0 + &f1;

        // Swap f0 with f1 and f1 with f2.
        f0 = mem::replace(&mut f1, f2);
    }

    f0
}
