use std::fmt;

pub enum ParseError {
    Empty,
    PosOverflow,
    NegOverflow,
    InvalidDigit,
}

impl ParseError {
    pub fn description(&self) -> &str {
        match self {
            ParseError::Empty => "cannot parse integer from empty input",
            ParseError::PosOverflow => "number is too large to fit in i64",
            ParseError::NegOverflow => "number is too small to fit in i64",
            ParseError::InvalidDigit => "invalid digit in input",
        }
    }
}

impl fmt::Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

/// Parse a string as a signed integer (`i64`).
pub fn parse_i64(src: &str) -> Result<i64, ParseError> {
    if src.is_empty() {
        return Err(ParseError::Empty);
    }

    let src = src.as_bytes();
    let (is_positive, digits) = match src[0] {
        b'+' | b'-' if src[1..].is_empty() => {
            return Err(ParseError::Empty);
        }
        b'+' => (true, &src[1..]),
        b'-' => (false, &src[1..]),
        _ => (true, src),
    };

    let mut result = 0i64;
    if is_positive {
        for &c in digits {
            let x = match c.wrapping_sub(b'0') {
                x @ 0..=9 => x as i64,
                _ if c == b'_' => continue,
                _ => return Err(ParseError::InvalidDigit),
            };
            result = match result.checked_mul(10) {
                Some(result) => result,
                None => return Err(ParseError::PosOverflow),
            };
            result = match result.checked_add(x) {
                Some(result) => result,
                None => return Err(ParseError::PosOverflow),
            };
        }
    } else {
        for &c in digits {
            let x = match c.wrapping_sub(b'0') {
                x @ 0..=9 => x as i64,
                _ if c == b'_' => continue,
                _ => return Err(ParseError::InvalidDigit),
            };
            result = match result.checked_mul(10) {
                Some(result) => result,
                None => return Err(ParseError::NegOverflow),
            };
            result = match result.checked_sub(x) {
                Some(result) => result,
                None => return Err(ParseError::NegOverflow),
            };
        }
    }

    Ok(result)
}
