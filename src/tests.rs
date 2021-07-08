use crate::*;

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

const QUICK_BROWN_FOX_ENC: &str =
    "8UADZCKFEOEDJOD2KC54EM-DX.CH8FSKDQ$D.OE44E5$CS44+8DK44OEC3EFGVCD2";
const QUICK_BROWN_FOX_DEC: &str = "The quick brown fox jumps over the lazy dog";
#[test]
fn decode_long_string() {
    assert_eq!(
        String::from_utf8(decode(QUICK_BROWN_FOX_ENC).unwrap()).unwrap(),
        QUICK_BROWN_FOX_DEC,
    )
}

#[test]
fn encode_hello_from_buffer() {
    assert_eq!(
        encode_from_buffer(vec![72, 101, 108, 108, 111, 33, 33]),
        "%69 VD92EX0"
    )
}

#[bench]
fn bench_encode_quick_brown_fox(b: &mut test::Bencher) {
    b.iter(|| {
        for _ in 0..100 {
            let text = String::from(QUICK_BROWN_FOX_DEC);
            let encoded = encode(&text);
            assert_eq!(encoded, QUICK_BROWN_FOX_ENC);
        }
    });
}

fn rbe<const N: usize>(bench: &mut test::Bencher) {
    use rand::*;
    let mut rng = thread_rng();
    let mut b = [0u8; N];
    rng.fill_bytes(&mut b);
    bench.iter(|| {
        let encoded = encode_from_buffer(&b[..]);
        assert!(encoded.is_ascii());
    });
}
#[bench]
fn bench_encode_random_0x10(b: &mut test::Bencher) {
    rbe::<0x10>(b);
}
#[bench]
fn bench_encode_random_0x100(b: &mut test::Bencher) {
    rbe::<0x100>(b);
}
#[bench]
fn bench_encode_random_0x1000(b: &mut test::Bencher) {
    rbe::<0x1000>(b);
}
#[bench]
fn bench_encode_random_0x10000(b: &mut test::Bencher) {
    rbe::<0x10000>(b);
}

#[bench]
fn bench_decode_quick_brown_fox(b: &mut test::Bencher) {
    b.iter(|| {
        for _ in 0..100 {
            let text = String::from(QUICK_BROWN_FOX_ENC);
            if let Ok(decoded) = decode(&text) {
                assert_eq!(decoded, QUICK_BROWN_FOX_DEC.as_bytes());
            }
        }
    });
}

// cursed code, really doesn't work

// fn rbd<const N: usize>(bench: &mut test::Bencher) {
//     use rand::{distributions::*, *};
//     use std::convert::TryFrom;
//     let mut rng = thread_rng();
//     let sample = Slice::new(&crate::alphabet::TABLE).unwrap();
//     let b: Vec<u8> = sample.sample_iter(rng).copied().take(N).collect();
//     let b: [u8; N] = TryFrom::try_from(b).unwrap();
//     bench.iter(|| {
//         let decoded = decode(&b[..]);
//         assert!(decoded.is_ok());
//     });
// }
// #[bench]
// fn bench_decode_random_3(b: &mut test::Bencher) {
//     rbd::<3>(b);
// }
// #[bench]
// fn bench_decode_random_30(b: &mut test::Bencher) {
//     rbd::<30>(b);
// }
// #[bench]
// fn bench_decode_random_3000(b: &mut test::Bencher) {
//     rbd::<3000>(b);
// }
// #[bench]
// fn bench_decode_random_3002(b: &mut test::Bencher) {
//     rbd::<3002>(b);
// }
