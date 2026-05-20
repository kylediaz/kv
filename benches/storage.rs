use criterion::{Criterion, criterion_group, criterion_main};

use kv::{resp::bytes_to_resp, storage::Storage};
use std::hint::black_box;

fn set(key: &'static str, value: i64) -> Vec<String> {
    vec!["SET".to_string(), key.to_string(), value.to_string()]
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Random SET", |b| {
        let mut storage = Storage::new();
        b.iter(|| storage.process_command(&set("key", 123)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
