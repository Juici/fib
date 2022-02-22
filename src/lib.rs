use rug::Integer;

fn lucas(n: u64) -> (Integer, Integer) {
    if n == 0 {
        let a = Integer::from(2u8);
        let b = Integer::from(1u8);
        return (a, b);
    }

    let (z, t) = lucas(n >> 1);

    match n & 3 {
        0 => {
            let zt = t * &z;

            let a = z.square() - 2u8;
            let b = zt - 1u8;

            (a, b)
        }
        1 => {
            let zt = z * &t;

            let a = zt - 1u8;
            let b = t.square() + 2u8;

            (a, b)
        }
        2 => {
            let zt = t * &z;

            let a = z.square() + 2u8;
            let b = zt + 1u8;

            (a, b)
        }
        3 => {
            let zt = z * &t;

            let a = zt + 1u8;
            let b = t.square() - 2u8;

            (a, b)
        }
        // This branch will be optimised out by the compiler.
        _ => unreachable!(),
    }
}

/// Computes the value of the `n`th term of the Fibonacci sequence.
pub fn fib(n: i64) -> Integer {
    if n == 0 {
        return Integer::ZERO;
    }

    let (mut a, b) = lucas(n.unsigned_abs() - 1);

    a <<= 1u32;
    a += &b;
    a = a.div_exact_u(5);

    // If `n` is negative and not odd, set the sign of the output to negative.
    if n < 0 && n & 1 == 0 {
        a = -a;
    }

    a
}
