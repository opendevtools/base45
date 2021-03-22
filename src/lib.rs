//! Encoder/decoder for base45 that is fully compatible with
//! [draft-faltstrom-base45-02.txt](https://www.ietf.org/id/draft-faltstrom-base45-02.txt)
//!
//! ```rust,no_run
//! # fn main() {
//!     let encoded = base45::encode("Hello!!");
//! # }
//! ```
mod alphabet;

use alphabet::Base45;

pub const SIZE: usize = 45;

fn divmod(x: usize, y: usize) -> (usize, usize) {
    ((x as f32 / y as f32).floor() as usize, x % y)
}

/// Encode a string to base45
///
/// ```rust,no_run
/// # fn main() {
/// let encoded = base45::encode("Hello!!");
/// # }
/// ```
pub fn encode(input: &str) -> String {
    let chunks: Vec<&[u8]> = input.as_bytes().chunks(2).collect();

    chunks
        .iter()
        .map(|v| match v {
            [first, second] => {
                let v = (*first as usize * 256) + *second as usize;
                let (e, rest) = divmod(v, SIZE.pow(2));
                let (d, c) = divmod(rest, SIZE);

                (Base45::encode(c), Base45::encode(d), Base45::encode(e))
            }
            [first] => {
                let (d, c) = divmod(*first as usize, SIZE);

                (Base45::encode(c), Base45::encode(d), None)
            }
            _ => (None, None, None),
        })
        .fold("".to_string(), |acc, (c, d, e)| match (c, d, e) {
            (Some(c), Some(d), Some(e)) => format!("{}{}{}{}", acc, c, d, e),
            (Some(c), Some(d), None) => format!("{}{}{}", acc, c, d),
            _ => acc,
        })
}

/// Decode a string from base45
///
/// ```rust,no_run
/// # fn main() -> Result<(), std::string::FromUtf8Error> {
/// let decoded = base45::decode("Hello!!")?;
/// # Ok(())
/// # }
/// ```
pub fn decode(input: &str) -> Result<String, std::string::FromUtf8Error> {
    let input_as_chars: Vec<Option<usize>> = input.chars().map(Base45::decode).collect();
    let chunked_input: Vec<&[Option<usize>]> = input_as_chars.chunks(3).collect();
    let mut output: Vec<u8> = vec![];

    chunked_input.iter().for_each(|v| match v {
        [Some(first), Some(second), Some(third)] => {
            let v = first + second * SIZE + third * SIZE.pow(2);
            let (x, y) = divmod(v, 256);

            output.push(x as u8);
            output.push(y as u8);
        }
        [Some(first), Some(second)] => {
            output.push((first + second * SIZE) as u8);
        }
        _ => unreachable!("Decode unsuccessful"),
    });

    String::from_utf8(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_ab() {
        assert_eq!(encode("AB"), "BB8")
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
        assert_eq!(decode("BB8").unwrap(), "AB")
    }

    #[test]
    fn decode_hello() {
        assert_eq!(decode("%69 VD92EX0").unwrap(), "Hello!!")
    }

    #[test]
    fn decode_base45() {
        assert_eq!(decode("UJCLQE7W581").unwrap(), "base-45")
    }

    #[test]
    fn decode_ietf() {
        assert_eq!(decode("QED8WEX0").unwrap(), "ietf!")
    }
}
