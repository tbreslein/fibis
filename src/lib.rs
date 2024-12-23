use std::marker::PhantomData;

use num_integer::Integer;

#[derive(Debug, Clone)]
pub struct BitSet<T: Integer + Copy, const N: usize, const MIN: usize, const MAX: usize> {
    data: [u64; N],
    len: usize,
    phantom: PhantomData<T>
}

impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> Default for BitSet<T, N, LOWER, UPPER> {
    fn default() -> Self {
        Self {
            data: [0_u64; N],
            len: 0,
            phantom: PhantomData
        }
    }
}

impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> BitSet<T, N, LOWER, UPPER> {
    const _N_FITS: () = assert!(N == ((UPPER - LOWER).div_ceil(64)));

    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        N
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let x = BitSet::<usize, 1, 0, 4>::new();
        assert_eq!(x.len(), 0);
    }
}
