//! Encoder/decoder for base45 that is fully compatible with
//! [draft-faltstrom-base45-02.txt](https://www.ietf.org/id/draft-faltstrom-base45-02.txt)
//!
//! ```rust,no_run
//! # fn main() {
//!     let encoded = base45::encode("Hello!!");
//! # }
//! ```

use std::error::Error;
use std::fmt::{Display, Formatter};

mod alphabet;

use alphabet::Base45;

const SIZE: u16 = 45;
const SIZE2: u16 = SIZE.pow(2);

fn divmod(x: u16, y: u16) -> (u8, u16) {
    ((x / y) as u8, x % y)
}

fn encode_buffer(input: Vec<&[u8]>) -> String {
    input
        .iter()
        .map(|v| match v {
            [first, second] => {
                let v = u16::from_be_bytes([*first, *second]);

                let (e, rest) = divmod(v, SIZE2);
                let (d, c) = divmod(rest, SIZE);

                (Base45::encode(c as u8), Base45::encode(d), Base45::encode(e))
            }
            [first] => {
                let v = *first as u16;
                let (d, c) = divmod(v, SIZE);

                (Base45::encode(c as u8), Base45::encode(d), None)
            }
            _ => (None, None, None),
        })
        .fold("".into(), |acc, (c, d, e)| match (c, d, e) {
            (Some(c), Some(d), Some(e)) => format!("{}{}{}{}", acc, c, d, e),
            (Some(c), Some(d), None) => format!("{}{}{}", acc, c, d),
            _ => acc,
        })
}

/// Encode a string to base45
///
/// The function takes a string containing only characters in
/// in the range U+0000 to U+00FF.
///
/// ```rust,no_run
/// # fn main() {
/// let encoded = base45::encode("Hello!!");
/// assert_eq!(encoded, "%69 VD92EX0");
/// # }
/// ```
pub fn encode(input: &str) -> String {
    encode_buffer(input.as_bytes().chunks(2).collect())
}

/// Encode a buffer to base45
///
/// The function takes a string containing only characters in
/// in the range U+0000 to U+00FF.
///
/// ```rust,no_run
/// # fn main() {
/// let encoded = base45::encode_from_buffer(vec![72,101,108,108,111,33,33]);
/// assert_eq!(encoded, "%69 VD92EX0");
/// # }
/// ```
pub fn encode_from_buffer(input: Vec<u8>) -> String {
    encode_buffer(input.chunks(2).collect())
}

/// The error type returned when the input is not a valid base45 string
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct DecodeError();

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid base45 string")
    }
}

impl Error for DecodeError {}

/// Decode a string from base45
///
/// Takes a base45 encoded string and returns a UTF-8 string on success
///
/// ```rust,no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let decoded = String::from_utf8(base45::decode("%69 VD92EX0")?)?;
/// assert_eq!(decoded, "Hello!!");
/// # Ok(())
/// # }
/// ```
pub fn decode(input: &str) -> Result<Vec<u8>, DecodeError> {
    let input_as_chars: Vec<Option<u8>> = input.chars().map(Base45::decode).collect();
    let chunked_input: Vec<&[Option<u8>]> = input_as_chars.chunks(3).collect();

    // Output will be shorter than input after decode, so we can allocate at least
    // input.len bytes upfront, avoiding reallocs
    let mut output = Vec::<u8>::with_capacity(input.len());

    for v in chunked_input {
        match v {
            [Some(first), Some(second), Some(third)] => {
                // We first need to work with an u32 register to properly detect overflow in
                // case of invalid input
                let first = *first as u32;
                let second = *second as u32;
                let third = *third as u32;
                let size = SIZE as u32;
                let size2 = SIZE2 as u32;

                let v = first + second * size + third * size2;

                if v > u16::MAX as u32 {
                    return Err(DecodeError());
                }

                // Now that the value is cleared, we can go back to u16
                let v = v as u16;

                v.to_be_bytes().iter().for_each(|b| output.push(*b));
            }
            [Some(first), Some(second)] => {
                output.push(first + second * SIZE as u8);
            }
            _ => return Err(DecodeError()),
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divmod_simple() {
        assert_eq!(divmod(100, 10), (10, 0));
        assert_eq!(divmod(101, 10), (10, 1));
        assert_eq!(divmod(9, 10), (0, 9));
    }

    #[test]
    fn encode_ab() {
        assert_eq!(encode("AB"), "BB8")
    }

    #[test]
    fn decode_fail() {
        assert!(decode(":::").is_err());
    }

    #[test]
    fn decode_fail_out_of_range() {
        assert!(decode(":::").is_err());
    }

    #[test]
    fn encode_hello() {
        assert_eq!(encode("Hello!!"), "%69 VD92EX0")
    }

    #[test]
    fn encode_base45() {
        assert_eq!(encode("base-45"), "UJCLQE7W581")
    }

    #[test]
    fn encode_long_string() {
        assert_eq!(
            encode("The quick brown fox jumps over the lazy dog"),
            "8UADZCKFEOEDJOD2KC54EM-DX.CH8FSKDQ$D.OE44E5$CS44+8DK44OEC3EFGVCD2",
        )
    }

    #[test]
    fn encode_unicode() {
        assert_eq!(encode("foo © bar 𝌆 baz"), "X.C82EIROA44GECH74C-J1/GUJCW2")
    }

    #[test]
    fn encode_emoji() {
        assert_eq!(encode("I ❤️  Rust"), "0B98TSD%K.ENY244JA QE")
    }

    #[test]
    fn encode_ietf() {
        assert_eq!(encode("ietf!"), "QED8WEX0")
    }

    #[test]
    fn decode_ab() {
        assert_eq!(String::from_utf8(decode("BB8").unwrap()).unwrap(), "AB")
    }

    #[test]
    fn decode_hello() {
        assert_eq!(
            String::from_utf8(decode("%69 VD92EX0").unwrap()).unwrap(),
            "Hello!!"
        )
    }

    #[test]
    fn decode_base45() {
        assert_eq!(
            String::from_utf8(decode("UJCLQE7W581").unwrap()).unwrap(),
            "base-45"
        )
    }

    #[test]
    fn decode_ietf() {
        assert_eq!(
            String::from_utf8(decode("QED8WEX0").unwrap()).unwrap(),
            "ietf!"
        )
    }

    #[test]
    fn decode_long_string() {
        assert_eq!(
            String::from_utf8(
                decode("8UADZCKFEOEDJOD2KC54EM-DX.CH8FSKDQ$D.OE44E5$CS44+8DK44OEC3EFGVCD2")
                    .unwrap()
            )
            .unwrap(),
            "The quick brown fox jumps over the lazy dog",
        )
    }

    #[test]
    fn encode_hello_from_buffer() {
        assert_eq!(
            encode_from_buffer(vec![72, 101, 108, 108, 111, 33, 33]),
            "%69 VD92EX0"
        )
    }
}
