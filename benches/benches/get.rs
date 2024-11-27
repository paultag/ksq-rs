use criterion::{criterion_group, criterion_main, Criterion};
use ksq::Tree;
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree");

    let mut tree = Tree::from(&[1, 1, 0]).unwrap();
    group.bench_function("get::empty", |b| {
        b.iter(|| {
            for idx in 0..tree.bits() {
                tree.get(idx);
            }
        });
    });

    group.bench_function("iter::empty", |b| {
        b.iter(|| {
            for v in tree.iter() {
                let _ = v;
            }
        });
    });

    {
        let mut rng = SmallRng::seed_from_u64(0xDEADBEEF);
        for _ in 0..512 {
            tree.set(rng.gen::<usize>() % tree.bits());
        }
    }

    group.bench_function("get::rand", |b| {
        b.iter(|| {
            for idx in 0..tree.bits() {
                tree.get(idx);
            }
        });
    });

    group.bench_function("iter::rand", |b| {
        b.iter(|| {
            for v in tree.iter() {
                let _ = v;
            }
        });
    });

    for idx in 0..tree.bits() {
        tree.set(idx);
    }

    group.bench_function("get::maxed", |b| {
        b.iter(|| {
            for idx in 0..tree.bits() {
                tree.get(idx);
            }
        });
    });

    group.bench_function("iter::maxed", |b| {
        b.iter(|| {
            for v in tree.iter() {
                let _ = v;
            }
        });
    });

    let tree = Tree::from(&[1, 1, 0]).unwrap();
    group.bench_function("set::all", |b| {
        let tree = tree.clone();
        b.iter(|| {
            let mut tree = tree.clone();
            for idx in 0..tree.bits() {
                tree.set(idx);
            }
        });
    });

    group.bench_function("set::same", |b| {
        let tree = tree.clone();
        b.iter(|| {
            let mut tree = tree.clone();
            tree.set(1024);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
