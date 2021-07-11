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
    let mut output = Vec::with_capacity(match input.len() & 1 {
        0 => input.len() * 2 / 3,
        1 => 1 + input.len() * 2 / 3,
        // SAFETY: it's one of 0 or 1. There are no other options.
        _ => unsafe { core::hint::unreachable_unchecked() },
    });

    #[inline(always)]
    fn core_fn([_0, _1, _2]: [u8; 3]) -> Result<[u8; 2], DecodeError> {
        let v = (_0 as u32) + (_1 as u32 * SIZE) + (_2 as u32) * SIZE_SIZE;
        if v <= (u16::MAX as _) {
            let x = (v >> 8) & 0xFF;
            let y = v & 0xFF;
            Ok([x as u8, y as u8])
        } else {
            Err(DecodeError)
        }
    }
    //short
    #[inline(always)]
    fn preproc([a, b, c]: [u8; 3]) -> Result<[u8; 3], DecodeError> {
        Ok([d(a)?, d(b)?, d(c)?])
    }
    //short
    #[inline(always)]
    fn d(v: u8) -> Result<u8, DecodeError> {
        alphabet::decode(v).ok_or(DecodeError)
    }

    #[cfg(feature = "array_chunks")]
    let (chunks, pre) = (input.array_chunks(), |&b| preproc(b));
    #[cfg(not(feature = "array_chunks"))]
    let (chunks, pre) = (input.chunks_exact(3), |slic: &[u8]| match slic {
        &[a, b, c] => preproc([a, b, c]),
        // SAFETY: chunks_exact ensures every `.next()` returns an effective &[T] where `.len` == 3
        _ => unsafe { core::hint::unreachable_unchecked() },
    });

    let remainder = chunks.remainder();
    for c in chunks.map(pre) {
        let xy = core_fn(c?)?;
        output.extend_from_slice(&xy);
    }
    match remainder {
        &[_0, _1] => output.push((d(_0)? as u32 + (d(_1)? as u32 * SIZE)) as u8),
        &[] => {}
        _ => return Err(DecodeError),
    }

    Ok(output)
}
