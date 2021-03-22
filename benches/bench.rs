use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn encode(c: &mut Criterion) {
    c.bench_function("encode long string", |b| {
        b.iter(|| base45::encode(black_box("The quick brown fox jumps over the lazy dog")))
    });
}

fn decode(c: &mut Criterion) {
    c.bench_function("decode long string", |b| {
        b.iter(|| {
            base45::decode(black_box(
                "8UADZCKFEOEDJOD2KC54EM-DX.CH8FSKDQ$D.OE44E5$CS44+8DK44OEC3EFGVCD2",
            ))
            .unwrap()
        })
    });
}

criterion_group!(benches, encode, decode);
criterion_main!(benches);
