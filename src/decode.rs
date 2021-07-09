use crate::alphabet::{self, SIZE, SIZE_SIZE};
use std::error::Error;
use std::fmt::{Display, Formatter};
/// The error type returned when the input is not a valid base45 string
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct DecodeError;

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
pub fn decode(input: impl AsRef<[u8]>) -> Result<Vec<u8>, DecodeError> {
    decode_intl(input.as_ref())
}
/// Internal function to decode a string from base45, to reduce code bloat.
fn decode_intl(input: &[u8]) -> Result<Vec<u8>, DecodeError> {
    // Setup indexing
    let input_as_idx: Vec<_> = input
        .iter()
        .map(|v| alphabet::decode(*v).ok_or(DecodeError))
        .collect::<Result<Vec<_>, DecodeError>>()?;
    let mut output = Vec::with_capacity(match input.len() & 1 {
        0 => input.len() * 2 / 3,
        1 => 1 + input.len() * 2 / 3,
        // SAFETY: it's one of 0 or 1. There are no other options.
        _ => unsafe { core::hint::unreachable_unchecked() },
    });

    #[inline(always)]
    fn core_fn([_0, _1, _2]: [u8; 3]) -> Result<[u8; 2], DecodeError> {
        let v = (_0 as u32) + (_1 as u32 * SIZE) + (_2 as u32) * SIZE_SIZE;
        if (0..=65792).contains(&v) {
            let x = (v >> 8) & 0xFF;
            let y = v & 0xFF;
            Ok([x as u8, y as u8])
        } else {
            Err(DecodeError)
        }
    }

    #[cfg(feature = "array_chunks")]
    let chunks = input_as_idx.array_chunks::<3>();
    #[cfg(not(feature = "array_chunks"))]
    let chunks = input_as_idx.chunks_exact(3);

    let remainder = chunks.remainder();
    for c in chunks {
        #[cfg(feature = "array_chunks")]
        let c = *c;
        // SAFETY: chunks_exact ensures every `.next()` returns an effective &[T] where `.len` == 3
        #[cfg(not(feature = "array_chunks"))]
        let c = unsafe {
            [
                *c.get_unchecked(0),
                *c.get_unchecked(1),
                *c.get_unchecked(2),
            ]
        };
        let [x, y] = core_fn(c)?;
        output.push(x);
        output.push(y);
    }
    match remainder {
        &[_0, _1] => output.push((_0 as u32 + (_1 as u32 * SIZE)) as u8),
        &[] => {}
        _ => return Err(DecodeError),
    }

    Ok(output)
}
