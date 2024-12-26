fn main() {
    divan::main();
}

#[divan::bench_group(sample_count = 100, sample_size = 500)]
mod construction_100 {
    use fibis::BitSet;
    use integer_hasher::IntSet;
    const LOWER: usize = 0;
    const UPPER: usize = 100;

    #[divan::bench]
    fn construction_intset() {
        IntSet::from_iter(LOWER..UPPER);
    }
    #[divan::bench]
    fn construction_bitset() {
        BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
    }
}

#[divan::bench_group(sample_count = 100, sample_size = 500)]
mod construction_1_000 {
    use fibis::BitSet;
    use integer_hasher::IntSet;
    const LOWER: usize = 0;
    const UPPER: usize = 1_000;

    #[divan::bench]
    fn construction_intset() {
        IntSet::from_iter(LOWER..UPPER);
    }
    #[divan::bench]
    fn construction_bitset() {
        BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
    }
}

#[divan::bench_group(sample_count = 100, sample_size = 500)]
mod construction_10_000 {
    use fibis::BitSet;
    use integer_hasher::IntSet;
    const LOWER: usize = 0;
    const UPPER: usize = 10_000;

    #[divan::bench]
    fn construction_intset() {
        IntSet::from_iter(LOWER..UPPER);
    }
    #[divan::bench]
    fn construction_bitset() {
        BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
    }
}

#[divan::bench_group(sample_count = 100, sample_size = 500)]
mod construction_100_000 {
    use fibis::BitSet;
    use integer_hasher::IntSet;
    const LOWER: usize = 0;
    const UPPER: usize = 100_000;

    #[divan::bench]
    fn construction_intset() {
        IntSet::from_iter(LOWER..UPPER);
    }
    #[divan::bench]
    fn construction_bitset() {
        BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
    }
}

#[divan::bench_group(sample_count = 100, sample_size = 500)]
mod construction_1_000_000 {
    use fibis::BitSet;
    use integer_hasher::IntSet;
    const LOWER: usize = 0;
    const UPPER: usize = 1_000_000;

    #[divan::bench]
    fn construction_intset() {
        IntSet::from_iter(LOWER..UPPER);
    }
    #[divan::bench]
    fn construction_bitset() {
        BitSet::<{ UPPER.div_ceil(64) }, LOWER, UPPER>::from_iter(0..UPPER);
    }
}
