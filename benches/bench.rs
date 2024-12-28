fn main() {
    divan::main();
}

mod insert {
    use rand::Rng;

    fn get_rng(lower: usize, upper: usize) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        (0..upper / 2)
            .map(|_| rng.gen_range(lower..upper))
            .collect()
    }

    mod insert_100 {
        use super::get_rng;
        use integer_hasher::{BuildIntHasher, IntSet};
        use std::collections::HashSet;

        const LOWER: usize = 0;
        const UPPER: usize = 100;

        #[divan::bench]
        fn contains_intset(bencher: divan::Bencher) {
            let s: IntSet<usize> =
                HashSet::with_capacity_and_hasher(UPPER, BuildIntHasher::default());
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(move || {
                let mut s = s.clone();
                for x in rng.iter() {
                    let _ = s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset(bencher: divan::Bencher) {
            let s = fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::default();
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec(bencher: divan::Bencher) {
            let s = fibis::vec::BitSet::<LOWER, UPPER>::default();
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec2(bencher: divan::Bencher) {
            let s = fibis::vec2::BitSet::new(LOWER, UPPER);
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
    }

    mod insert_1_000 {
        use super::get_rng;
        use integer_hasher::{BuildIntHasher, IntSet};
        use std::collections::HashSet;

        const LOWER: usize = 0;
        const UPPER: usize = 1_000;

        #[divan::bench]
        fn contains_intset(bencher: divan::Bencher) {
            let s: IntSet<usize> =
                HashSet::with_capacity_and_hasher(UPPER, BuildIntHasher::default());
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(move || {
                let mut s = s.clone();
                for x in rng.iter() {
                    let _ = s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset(bencher: divan::Bencher) {
            let s = fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::default();
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec(bencher: divan::Bencher) {
            let s = fibis::vec::BitSet::<LOWER, UPPER>::default();
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec2(bencher: divan::Bencher) {
            let s = fibis::vec2::BitSet::new(LOWER, UPPER);
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
    }

    mod insert_10_000 {
        use super::get_rng;
        use integer_hasher::{BuildIntHasher, IntSet};
        use std::collections::HashSet;

        const LOWER: usize = 0;
        const UPPER: usize = 10_000;

        #[divan::bench]
        fn contains_intset(bencher: divan::Bencher) {
            let s: IntSet<usize> =
                HashSet::with_capacity_and_hasher(UPPER, BuildIntHasher::default());
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(move || {
                let mut s = s.clone();
                for x in rng.iter() {
                    let _ = s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset(bencher: divan::Bencher) {
            let s = fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::default();
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec(bencher: divan::Bencher) {
            let s = fibis::vec::BitSet::<LOWER, UPPER>::default();
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec2(bencher: divan::Bencher) {
            let s = fibis::vec2::BitSet::new(LOWER, UPPER);
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
    }

    mod insert_100_000 {
        use super::get_rng;
        use integer_hasher::{BuildIntHasher, IntSet};
        use std::collections::HashSet;

        const LOWER: usize = 0;
        const UPPER: usize = 100_000;

        #[divan::bench]
        fn contains_intset(bencher: divan::Bencher) {
            let s: IntSet<usize> =
                HashSet::with_capacity_and_hasher(UPPER, BuildIntHasher::default());
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(move || {
                let mut s = s.clone();
                for x in rng.iter() {
                    let _ = s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset(bencher: divan::Bencher) {
            let s = fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::default();
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec(bencher: divan::Bencher) {
            let s = fibis::vec::BitSet::<LOWER, UPPER>::default();
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec2(bencher: divan::Bencher) {
            let s = fibis::vec2::BitSet::new(LOWER, UPPER);
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
    }

    mod insert_1_000_000 {
        use super::get_rng;
        use integer_hasher::{BuildIntHasher, IntSet};
        use std::collections::HashSet;

        const LOWER: usize = 0;
        const UPPER: usize = 1_000_000;

        #[divan::bench]
        fn contains_intset(bencher: divan::Bencher) {
            let s: IntSet<usize> =
                HashSet::with_capacity_and_hasher(UPPER, BuildIntHasher::default());
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(move || {
                let mut s = s.clone();
                for x in rng.iter() {
                    let _ = s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset(bencher: divan::Bencher) {
            let s = fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::default();
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec(bencher: divan::Bencher) {
            let s = fibis::vec::BitSet::<LOWER, UPPER>::default();
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec2(bencher: divan::Bencher) {
            let s = fibis::vec2::BitSet::new(LOWER, UPPER);
            let rng = get_rng(LOWER, UPPER);
            bencher.bench(|| {
                let mut s = s.clone();
                for x in rng.iter() {
                    s.insert(*x);
                }
            });
        }
    }
}

mod contains {
    use rand::Rng;

    fn get_checks(lower: usize, upper: usize) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        (0..upper / 2)
            .map(|_| rng.gen_range(lower..upper))
            .collect()
    }

    mod contains_100 {
        use integer_hasher::IntSet;

        use super::get_checks;
        const LOWER: usize = 0;
        const UPPER: usize = 100;

        #[divan::bench]
        fn contains_intset(bencher: divan::Bencher) {
            let s = IntSet::from_iter(LOWER..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset(bencher: divan::Bencher) {
            let s = fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec(bencher: divan::Bencher) {
            let s = fibis::vec::BitSet::<LOWER, UPPER>::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec2(bencher: divan::Bencher) {
            let s = fibis::vec2::BitSet::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
    }

    mod contains_1_000 {
        use integer_hasher::IntSet;

        use super::get_checks;
        const LOWER: usize = 0;
        const UPPER: usize = 1_000;

        #[divan::bench]
        fn contains_intset(bencher: divan::Bencher) {
            let s = IntSet::from_iter(LOWER..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset(bencher: divan::Bencher) {
            let s = fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec(bencher: divan::Bencher) {
            let s = fibis::vec::BitSet::<LOWER, UPPER>::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec2(bencher: divan::Bencher) {
            let s = fibis::vec2::BitSet::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
    }

    mod contains_10_000 {
        use integer_hasher::IntSet;

        use super::get_checks;
        const LOWER: usize = 0;
        const UPPER: usize = 10_000;

        #[divan::bench]
        fn contains_intset(bencher: divan::Bencher) {
            let s = IntSet::from_iter(LOWER..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset(bencher: divan::Bencher) {
            let s = fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec(bencher: divan::Bencher) {
            let s = fibis::vec::BitSet::<LOWER, UPPER>::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec2(bencher: divan::Bencher) {
            let s = fibis::vec2::BitSet::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
    }

    mod contains_100_000 {
        use integer_hasher::IntSet;

        use super::get_checks;
        const LOWER: usize = 0;
        const UPPER: usize = 100_000;

        #[divan::bench]
        fn contains_intset(bencher: divan::Bencher) {
            let s = IntSet::from_iter(LOWER..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset(bencher: divan::Bencher) {
            let s = fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec(bencher: divan::Bencher) {
            let s = fibis::vec::BitSet::<LOWER, UPPER>::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec2(bencher: divan::Bencher) {
            let s = fibis::vec2::BitSet::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
    }
    mod contains_1_000_000 {
        use integer_hasher::IntSet;

        use super::get_checks;
        const LOWER: usize = 0;
        const UPPER: usize = 1_000_000;

        #[divan::bench]
        fn contains_intset(bencher: divan::Bencher) {
            let s = IntSet::from_iter(LOWER..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset(bencher: divan::Bencher) {
            let s = fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec(bencher: divan::Bencher) {
            let s = fibis::vec::BitSet::<LOWER, UPPER>::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
        #[divan::bench]
        fn contains_bitset_vec2(bencher: divan::Bencher) {
            let s = fibis::vec2::BitSet::from_iter(0..UPPER);
            let checks = get_checks(LOWER, UPPER);
            bencher.bench(|| {
                for x in checks.iter() {
                    let _ = s.contains(*x);
                }
            });
        }
    }
}

mod construction {
    mod construction_100 {
        use integer_hasher::IntSet;
        const LOWER: usize = 0;
        const UPPER: usize = 100;

        #[divan::bench]
        fn construction_intset() {
            IntSet::from_iter(LOWER..UPPER);
        }
        #[divan::bench]
        fn construction_bitset() {
            fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
        }
        #[divan::bench]
        fn construction_bitset_vec() {
            fibis::vec::BitSet::<LOWER, UPPER>::from_iter(0..UPPER);
        }
        #[divan::bench]
        fn construction_bitset_vec2() {
            fibis::vec2::BitSet::from_iter(0..UPPER);
        }
    }

    mod construction_1_000 {
        use integer_hasher::IntSet;
        const LOWER: usize = 0;
        const UPPER: usize = 1_000;

        #[divan::bench]
        fn construction_intset() {
            IntSet::from_iter(LOWER..UPPER);
        }
        #[divan::bench]
        fn construction_bitset() {
            fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
        }
        #[divan::bench]
        fn construction_bitset_vec() {
            fibis::vec::BitSet::<LOWER, UPPER>::from_iter(0..UPPER);
        }
        #[divan::bench]
        fn construction_bitset_vec2() {
            fibis::vec2::BitSet::from_iter(0..UPPER);
        }
    }

    mod construction_10_000 {
        use integer_hasher::IntSet;
        const LOWER: usize = 0;
        const UPPER: usize = 10_000;

        #[divan::bench]
        fn construction_intset() {
            IntSet::from_iter(LOWER..UPPER);
        }
        #[divan::bench]
        fn construction_bitset() {
            fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
        }
        #[divan::bench]
        fn construction_bitset_vec() {
            fibis::vec::BitSet::<LOWER, UPPER>::from_iter(0..UPPER);
        }
        #[divan::bench]
        fn construction_bitset_vec2() {
            fibis::vec2::BitSet::from_iter(0..UPPER);
        }
    }

    mod construction_100_000 {
        use integer_hasher::IntSet;
        const LOWER: usize = 0;
        const UPPER: usize = 100_000;

        #[divan::bench]
        fn construction_intset() {
            IntSet::from_iter(LOWER..UPPER);
        }
        #[divan::bench]
        fn construction_bitset() {
            fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
        }
        #[divan::bench]
        fn construction_bitset_vec() {
            fibis::vec::BitSet::<LOWER, UPPER>::from_iter(0..UPPER);
        }
        #[divan::bench]
        fn construction_bitset_vec2() {
            fibis::vec2::BitSet::from_iter(0..UPPER);
        }
    }

    //#[divan::bench_group(sample_count = 100, sample_size = 500)]
    mod construction_1_000_000 {
        use integer_hasher::IntSet;
        const LOWER: usize = 0;
        const UPPER: usize = 1_000_000;

        #[divan::bench]
        fn construction_intset() {
            IntSet::from_iter(LOWER..UPPER);
        }
        #[divan::bench]
        fn construction_bitset() {
            fibis::BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
        }
        #[divan::bench]
        fn construction_bitset_vec() {
            fibis::vec::BitSet::<LOWER, UPPER>::from_iter(0..UPPER);
        }
        #[divan::bench]
        fn construction_bitset_vec2() {
            fibis::vec2::BitSet::from_iter(0..UPPER);
        }
    }
}
