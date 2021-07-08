use crate::alphabet::{self, SIZE, SIZE_SIZE};
fn divmod<const N: usize>(x: usize) -> (usize, usize) {
    (x / N as usize, x % N)
}

fn encode_buffer(input: &[u8]) -> String {
    // setup
    #[cfg(feature = "array_chunks")]
    let input = input.array_chunks::<2>();
    #[cfg(not(feature = "array_chunks"))]
    let input = input.chunks_exact(2);

    let mut s = String::with_capacity(input.len() + ((input.len() + 1) / 2));

    // Core function
    #[inline(always)]
    fn core_fn([_0, _1]: [u8; 2], s: &mut String) {
        let v = (_0 as usize * 256) + _1 as usize;
        let (e, rest) = divmod::<SIZE_SIZE>(v);
        let (d, c) = divmod::<SIZE>(rest);

        for b in [c, d, e] {
            if let Some(ch) = alphabet::encode(b) {
                s.push(ch as _);
            }
        }
    }
    // take remainder AoT
    let rem = input.remainder();
    for c in input {
        #[cfg(feature = "array_chunks")]
        let c = *c;
        #[cfg(not(feature = "array_chunks"))]
        let c = unsafe { [*c.get_unchecked(0), *c.get_unchecked(1)] };
        core_fn(c, &mut s);
    }
    // Final byte
    if let &[_0] = rem {
        let (d, c) = divmod::<SIZE>(_0 as usize);

        for b in [c, d] {
            if let Some(ch) = alphabet::encode(b) {
                s.push(ch as _);
            }
        }
    }
    s
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
pub fn encode(input: impl AsRef<[u8]>) -> String {
    encode_buffer(input.as_ref())
}

/// Encode a buffer to base45
///
/// The function takes an arbitrary buffer and encodes it to base45.
///
/// ```rust,no_run
/// # fn main() {
/// let encoded = base45::encode_from_buffer(vec![72,101,108,108,111,33,33]);
/// assert_eq!(encoded, "%69 VD92EX0");
/// # }
/// ```
pub fn encode_from_buffer(input: impl AsRef<[u8]>) -> String {
    encode_buffer(input.as_ref())
}
