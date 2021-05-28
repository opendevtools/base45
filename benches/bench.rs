use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn encode(c: &mut Criterion) {
    c.bench_function("encode long string", |b| {
        b.iter(|| base45::encode(black_box("The quick brown fox jumps over the lazy dog")))
    });
}

fn encode_from_buffer(c: &mut Criterion) {
    c.bench_function("encode long string from buffer", |b| {
        b.iter(|| {
            base45::encode_from_buffer(black_box(vec![
                84, 104, 101, 32, 113, 117, 105, 99, 107, 32, 98, 114, 111, 119, 110, 32, 102, 111,
                120, 32, 106, 117, 109, 112, 115, 32, 111, 118, 101, 114, 32, 116, 104, 101, 32,
                108, 97, 122, 121, 32, 100, 111, 103,
            ]))
        })
    });
}

fn decode(c: &mut Criterion) {
    c.bench_function("decode long string", |b| {
        b.iter(|| {
            String::from_utf8(
                base45::decode(black_box(
                    "8UADZCKFEOEDJOD2KC54EM-DX.CH8FSKDQ$D.OE44E5$CS44+8DK44OEC3EFGVCD2",
                ))
                .unwrap(),
            )
            .unwrap()
        })
    });
}

criterion_group!(benches, encode, encode_from_buffer, decode);
criterion_main!(benches);
