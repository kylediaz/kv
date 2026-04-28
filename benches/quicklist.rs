use criterion::{Criterion, criterion_group, criterion_main};

use kv::quicklist::{Dequeue, Quicklist};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("quicklist i32", |b| {
        b.iter(|| {
            let mut list: Quicklist<i32> = Quicklist::new();
            // This will generate a structure ...7531246...
            for n in 1..=1000 {
                if n % 2 == 0 {
                    list.rpush(black_box(n));
                } else {
                    list.lpush(black_box(n));
                }
            }
            for n in (1..=1000).rev() {
                let v;
                if n % 2 == 0 {
                    v = list.rpop();
                } else {
                    v = list.lpop();
                }
                assert_eq!(v.unwrap(), n);
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
