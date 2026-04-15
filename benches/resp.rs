use criterion::{Criterion, criterion_group, criterion_main};

use kv::resp::bytes_to_resp;
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("resp ping", |b| {
        b.iter(|| bytes_to_resp(black_box("+PING".as_bytes()), black_box(&mut 0)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
