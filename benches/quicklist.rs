use criterion::{Criterion, criterion_group, criterion_main};

use kv::quicklist::{Dequeue, Quicklist};
use rand::{Rng, SeedableRng, rngs::ChaCha8Rng};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("quicklist i32 stairs", |b| {
        b.iter(|| {
            let mut list: Quicklist<i32> = Quicklist::new();
            // This will generate a structure ...7 5 3 1 2 4 6...
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

    c.bench_function("quicklist u32 seeded random", |b| {
        b.iter(|| {
            let mut rng = ChaCha8Rng::seed_from_u64(10);
            let mut list: Quicklist<u32> = Quicklist::new();
            for n in 1..=10_000 {
                let action = rng.next_u64();
                // Use the 32 least significant bits to determine the action
                // Use the other 32 bits as the potentially pushed value
                let left: bool = (action & 1) == 1;
                // 75% chance of pushing
                let push: bool = (action & 0b110) != 0;
                let value = (action >> 32) as u32;
                if left && push {
                    list.lpush(black_box(value));
                } else if !left && push {
                    list.rpush(black_box(n));
                } else if left && !push {
                    list.lpop();
                } else if !left && !push {
                    list.rpop();
                }
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
