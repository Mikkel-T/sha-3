use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sha3::{sha3_224, sha3_256, sha3_384, sha3_512, shake128, shake256};

fn sha3_benchmark(c: &mut Criterion) {
    c.bench_function("sha3_224", |b| b.iter(|| sha3_224(black_box(""))));
    c.bench_function("sha3_256", |b| b.iter(|| sha3_256(black_box(""))));
    c.bench_function("sha3_384", |b| b.iter(|| sha3_384(black_box(""))));
    c.bench_function("sha3_512", |b| b.iter(|| sha3_512(black_box(""))));
}

fn shake_benchmark(c: &mut Criterion) {
    c.bench_function("shake128", |b| b.iter(|| shake128(black_box(""), 256)));
    c.bench_function("shake256", |b| b.iter(|| shake256(black_box(""), 512)));
}

criterion_group!(
  name = benches;
  config = Criterion::default().sample_size(250);
  targets = sha3_benchmark, shake_benchmark
);
criterion_main!(benches);
