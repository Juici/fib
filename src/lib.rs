use num_bigint::BigUint;
use num_traits::{One, Zero};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rustc_hash::FxHashMap;

const INITIAL_CAPACITY: usize = 100;

static CACHE: Lazy<RwLock<FxHashMap<u128, BigUint>>> = Lazy::new(|| {
    RwLock::new(FxHashMap::with_capacity_and_hasher(INITIAL_CAPACITY, Default::default()))
});

/// Computes the value of the `n`th term of the Fibonacci sequence.
pub fn fib(n: u128) -> BigUint {
    if let Some(value) = CACHE.read_recursive().get(&n) {
        return value.clone();
    }
    fn inner(n: u128) -> BigUint {
        match n {
            0 => return BigUint::zero(),
            1 | 2 => return BigUint::one(),
            _ => {}
        }

        if n & 1 != 0 {
            let k = (n + 1) / 2;

            let mut f0 = fib(k);
            let mut f1 = fib(k - 1);

            let mut tmp = f0.clone();
            f0 *= &tmp;

            tmp.clone_from(&f1);
            f1 *= &tmp;

            f0 + f1
        } else {
            let k = n / 2;

            let f0 = fib(k);
            let mut f1 = fib(k - 1);

            f1 <<= 1u8;
            f1 += &f0;
            f1 *= &f0;

            f1
        }
    }
    let value = inner(n);
    CACHE.write().entry(n).insert_entry(value).get().clone()
}
