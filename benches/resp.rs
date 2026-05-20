use criterion::{Criterion, criterion_group, criterion_main};

use kv::resp::bytes_to_resp;
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("RESP PING", |b| {
        b.iter(|| bytes_to_resp(black_box("+PING".as_bytes()), black_box(&mut 0)))
    });
    c.bench_function("RESP SET", |b| {
        b.iter(|| {
            bytes_to_resp(
                black_box(
                    "*3\r\n$3\r\nSET\r\n$20\r\ntest-set-and-get-key\r\n$5\r\nvalue\r\n".as_bytes(),
                ),
                black_box(&mut 0),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
