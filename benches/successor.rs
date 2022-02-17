use std::collections::BTreeMap;

use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("successor");
    let mut rng = rand::thread_rng();
    for num_keys in [10_000, 100_000, 200_000, 300_000, 400_000] {
        // Generate random keys.
        let keys: Vec<u64> =
            (0..num_keys).map(|_| rng.gen_range(0..=u64::MAX)).collect();

        // Insert the same keys into each implementation.
        let mut tree = cache_oblivious::VanEmdeBoasTree::<u64, u64>::new();
        for k in &keys {
            tree.insert(*k, *k);
        }

        let mut b_tree: BTreeMap<u64, u64> = BTreeMap::new();
        for k in &keys {
            b_tree.insert(*k, *k);
        }

        // Benchmark each implementation.
        group.bench_with_input(
            BenchmarkId::new("VanEmdeBoasTree", num_keys),
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

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
