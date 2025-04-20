use std::{collections::BTreeMap, time::Duration};

use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};
use rand::Rng;

fn bench_successor_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("successor_single");
    let mut rng = rand::rng();
    for num_keys in [
        10_000, 100_000, 500_000, 1_000_000, 10_000_000, 20_000_000,
        30_000_000, 40_000_000,
    ] {
        // Generate random keys.
        let keys: Vec<u64> = (0..num_keys)
            .map(|_| rng.random_range(0..=u64::MAX))
            .collect();

        // Insert the same keys into each implementation.
        let mut tree = veb_tree::VebTreeMap::<u64, u64>::new();
        for k in &keys {
            tree.insert(*k, *k);
        }

        let mut b_tree: BTreeMap<u64, u64> = BTreeMap::new();
        for k in &keys {
            b_tree.insert(*k, *k);
        }

        // Benchmark each implementation.
        group.bench_with_input(
            BenchmarkId::new("VebTreeMap", num_keys),
            &num_keys,
            |b, _i| {
                b.iter(|| {
                    tree.successor(&black_box(1u64 << 32));
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("BTreeMap", num_keys),
            &num_keys,
            |b, _i| {
                b.iter(|| {
                    b_tree.range(&black_box(1u64 << 32)..).next();
                })
            },
        );
    }
    group.finish();
}

macro_rules! bench_successor_key {
    ($fn_name: ident, $name: expr, $key_ty: ty, $max_key: expr, $sort_keys: expr) => {
        fn $fn_name(c: &mut Criterion) {
            let mut group = c.benchmark_group($name);
            group.measurement_time(Duration::from_secs(20));
            let mut rng = rand::rng();

            for num_keys in [
                10_000, 100_000, 500_000, 1_000_000, 10_000_000, 20_000_000,
                30_000_000, 40_000_000,
            ] {
                let num_targets = 100_00;

                if num_keys >= 500_000 {
                    // Reduce sample size for larger inputs.
                    group.sample_size(50);
                }

                // Generate random keys.
                let keys: Vec<$key_ty> = (0..num_keys).collect();

                // Insert the same keys into each implementation.
                let mut tree = veb_tree::VebTreeMap::<$key_ty, u64>::new();
                for k in &keys {
                    tree.insert(*k, *k as u64);
                }

                let mut b_tree: BTreeMap<$key_ty, u64> = BTreeMap::new();
                for k in &keys {
                    b_tree.insert(*k, *k as u64);
                }

                // Generate random keys to search for.
                let mut target_keys: Vec<$key_ty> = (0..num_targets)
                    .map(|_| rng.random_range(0..=$max_key))
                    .collect();

                if $sort_keys {
                    // Look them up in sorted order.
                    target_keys.sort_unstable();
                }

                // Benchmark each implementation.
                group.bench_with_input(
                    BenchmarkId::new("VebTreeMap", num_keys),
                    &num_keys,
                    |b, _i| {
                        b.iter(|| {
                            for target in &target_keys {
                                tree.successor(target);
                            }
                        })
                    },
                );

                group.bench_with_input(
                    BenchmarkId::new("BTreeMap", num_keys),
                    &num_keys,
                    |b, _i| {
                        b.iter(|| {
                            for target in &target_keys {
                                b_tree.range(target..).next();
                            }
                        })
                    },
                );
            }
            group.finish();
        }
    };
}

bench_successor_key!(
    bench_successor_multiple_in_order,
    "successor_multiple_in_order",
    u64,
    u64::MAX,
    true
);

bench_successor_key!(
    bench_successor_multiple_random_order,
    "successor_multiple_random_order",
    u64,
    u64::MAX,
    false
);

bench_successor_key!(
    bench_successor_multiple_random_order_u32,
    "successor_multiple_random_order_u32",
    u32,
    u32::MAX,
    false
);

criterion_group!(
    benches,
    bench_successor_single,
    bench_successor_multiple_in_order,
    bench_successor_multiple_random_order,
    bench_successor_multiple_random_order_u32,
);
criterion_main!(benches);
