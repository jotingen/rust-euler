use euler::primes;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("primes 0 to 10000", |b| {
        b.iter(|| primes::Primes::new().primes_between_u32(black_box(0), black_box(10000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
