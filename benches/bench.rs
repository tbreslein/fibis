#![allow(clippy::unit_arg)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fibis::BitSet;
use integer_hasher::{BuildIntHasher, IntSet};
use rand::Rng;
use std::collections::HashSet;

fn construction<const LOWER: usize, const UPPER: usize>(c: &mut Criterion) {
    c.bench_function(&format!("construct intset {}", UPPER).to_string(), |b| {
        b.iter(|| black_box(IntSet::from_iter(LOWER..=UPPER)))
    });
    c.bench_function(&format!("construct BitSet {}", UPPER).to_string(), |b| {
        b.iter(|| black_box(BitSet::<LOWER, UPPER>::from_iter(LOWER..=UPPER)))
    });
}

fn get_rng(lower: usize, upper: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    (0..upper / 2)
        .map(|_| rng.gen_range(lower..upper))
        .collect()
}

fn insertion<const LOWER: usize, const UPPER: usize>(c: &mut Criterion) {
    let rng = get_rng(LOWER, UPPER);
    let mut s: IntSet<usize> = HashSet::with_capacity_and_hasher(UPPER, BuildIntHasher::default());
    c.bench_function(&format!("insert intset {}", UPPER).to_string(), |b| {
        b.iter(|| {
            for x in rng.iter() {
                black_box(s.insert(*x));
            }
        })
    });
    let mut s = BitSet::<LOWER, UPPER>::new();
    c.bench_function(&format!("insert BitSet {}", UPPER).to_string(), |b| {
        b.iter(|| {
            for x in rng.iter() {
                black_box(s.insert(*x));
            }
        })
    });
}

fn contains<const LOWER: usize, const UPPER: usize>(c: &mut Criterion) {
    let rng = get_rng(LOWER, UPPER);
    let s = IntSet::from_iter(LOWER..=UPPER);
    c.bench_function(&format!("contains intset {}", UPPER).to_string(), |b| {
        b.iter(|| {
            for x in rng.iter() {
                black_box(s.contains(x));
            }
        })
    });
    let s = BitSet::<LOWER, UPPER>::from_iter(LOWER..=UPPER);
    c.bench_function(&format!("contains BitSet {}", UPPER).to_string(), |b| {
        b.iter(|| {
            for x in rng.iter() {
                black_box(s.contains(*x));
            }
        })
    });
}

criterion_group!(
    construction_bench,
    construction<0, 100>,
    construction<0, 1_000>,
    construction<0, 10_000>,
    construction<0, 100_000>,
    construction<0, 1_000_000>
);

criterion_group!(
    insertion_bench,
    insertion<0, 100>,
    insertion<0, 1_000>,
    insertion<0, 10_000>,
    insertion<0, 100_000>,
    insertion<0, 1_000_000>
);

criterion_group!(
    contains_bench,
    contains<0, 100>,
    contains<0, 1_000>,
    contains<0, 10_000>,
    contains<0, 100_000>,
    contains<0, 1_000_000>
);

criterion_main!(construction_bench, insertion_bench, contains_bench);
