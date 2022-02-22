use std::fmt;

pub enum ParseError {
    Empty,
    Negative,
    Overflow,
    InvalidDigit,
}

impl fmt::Display for ParseError {
    #[inline]
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

/// Parse a string as a `u128`.
pub fn parse_u128(src: &str) -> Result<u128, ParseError> {
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

    let mut result = 0u128;
    for &c in digits {
        let x = match c.wrapping_sub(b'0') {
            x @ 0..=9 => x as u128,
            _ if c == b'_' => continue,
            _ => return Err(ParseError::InvalidDigit),
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
