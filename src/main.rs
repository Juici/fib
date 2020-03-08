use std::{env, mem, process, fmt};

use num_bigint::BigUint;
use num_traits::{One, Zero};

enum ParseError {
    Empty,
    Negative,
    Overflow,
    InvalidDigit,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            ParseError::Empty => "cannot parse integer from empty input",
            ParseError::Negative => "cannot calculate fibonacci for a negative number",
            ParseError::Overflow => "number is too large to fit in usize",
            ParseError::InvalidDigit => "invalid digit in input",
        };
        f.write_str(s)
    }
}

fn parse_usize(src: &str) -> Result<usize, ParseError> {
    if src.is_empty() {
        return Err(ParseError::Empty);
    }

    let src = src.as_bytes();
    let digits = match src[0] {
        b'+' => &src[1..],
        b'-' => return Err(ParseError::Negative),
        _ => src,
    };

    if digits.is_empty() {
        return Err(ParseError::Empty);
    }

    let mut result = 0usize;
    for &c in digits {
        let x = match (c as char).to_digit(10) {
            Some(x) => x as usize,
            None => return Err(ParseError::InvalidDigit),
        };
        result = match result.checked_mul(10) {
            Some(result) => result,
            None => return Err(ParseError::Overflow),
        };
        result = match result.checked_add(x) {
            Some(result) => result,
            None => return Err(ParseError::Overflow),
        };
    }

    Ok(result)
}

fn fib(n: usize) -> BigUint {
    let mut f0 = BigUint::zero();
    let mut f1 = BigUint::one();

    for _ in 0..n {
        let f2 = f0 + &f1;

        // Swap f0 with f1 and f1 with f2.
        f0 = mem::replace(&mut f1, f2);
    }

    f0
}

fn main() {
    let mut exit_code = 0i32;

    for arg in env::args().skip(1) {
        let n = match parse_usize(&arg) {
            Ok(n) => n,
            Err(err) => {
                eprintln!("error: {}: {}", err, arg);
                exit_code = 1;
                continue;
            }
        };

        println!("{}", fib(n));
    }

    process::exit(exit_code);
}
