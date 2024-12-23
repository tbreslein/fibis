use std::{marker::PhantomData, ops::{BitAnd, BitOr, BitXor, Sub}, panic::{RefUnwindSafe, UnwindSafe}};

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

    pub fn clear(&mut self) {
        for x in self.data.iter_mut() {
            *x = 0_u64;
        }
    }

    pub fn contains(&self) -> bool {
        todo!();
    }

    pub fn insert(&mut self) -> bool {
        todo!();
    }

    pub fn remove(&mut self) -> bool {
        todo!();
    }

    pub fn take(&mut self) -> bool {
        todo!();
    }

    pub fn retain(&mut self, _f: impl FnMut(T) -> bool) {
        todo!()
    }

    /// Visits the values representing the difference, i.e., the values that are in self but not in other.
    pub fn difference(&self, _other: &Self) -> Difference<T> {
        todo!()
    }

    /// Visits the values representing the symmetric difference, i.e., the values that are in self or in other but not in both.
    pub fn symmetric_difference(&self, _other: &Self) -> SymmetricDifference<T> {
        todo!()
    }

    /// Visits the values representing the intersection, i.e., the values that are both in self and other.
    ///
    /// When an equal element is present in self and other then the resulting Intersection may yield references to one or the other. This can be relevant if T contains fields which are not compared by its Eq implementation, and may hold different value between the two equal copies of T in the two sets.
    pub fn intersection(&self, _other: &Self) -> Intersection<T> {
        todo!()
    }

    /// Visits the values representing the union, i.e., all the values in self or other, without duplicates.
    pub fn union(&self, _other: &Self) -> Union<T> {
        todo!()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        todo!()
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> BitAnd for BitSet<T, N, LOWER, UPPER> {
    type Output = Self;
    fn bitand(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> BitOr for BitSet<T, N, LOWER, UPPER> {
    type Output = Self;
    fn bitor(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> BitXor for BitSet<T, N, LOWER, UPPER> {
    type Output = Self;
    fn bitxor(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> Extend<T> for BitSet<T, N, LOWER, UPPER> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, _iter: I) {
        todo!()
    }
}

impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize, const M: usize> From<[T; M]> for BitSet<T, N, LOWER, UPPER> {
    fn from(_value: [T; M]) -> Self {
        todo!()
    }
}

impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> FromIterator<T> for BitSet<T, N, LOWER, UPPER> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        todo!()
    }
}

impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> IntoIterator for BitSet<T, N, LOWER, UPPER> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> IntoIterator for &BitSet<T, N, LOWER, UPPER> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> PartialEq for &BitSet<T, N, LOWER, UPPER> {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> Eq for &BitSet<T, N, LOWER, UPPER> { }

impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> Sub for &BitSet<T, N, LOWER, UPPER> {
    type Output = Self;
    fn sub(self, _rhs: Self) -> Self::Output {
        // basically just the difference but as a new Set
        todo!()
    }
}

unsafe impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> Send for BitSet<T, N, LOWER, UPPER> {}
unsafe impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> Sync for BitSet<T, N, LOWER, UPPER> {}
impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> RefUnwindSafe for BitSet<T, N, LOWER, UPPER> {}
impl<T: Integer + Copy, const N: usize, const LOWER: usize, const UPPER: usize> UnwindSafe for BitSet<T, N, LOWER, UPPER> {}

pub struct Difference<T: Integer + Copy> {
    phantom: PhantomData<T>
}
pub struct SymmetricDifference<T: Integer + Copy> {
    phantom: PhantomData<T>
}
pub struct Intersection<T: Integer + Copy> {
    phantom: PhantomData<T>
}
pub struct Union<T: Integer + Copy> {
    phantom: PhantomData<T>
}
pub struct IntoIter<T: Integer + Copy> {
    phantom: PhantomData<T>
}

impl<T: Integer + Copy> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // TODO: move this into doctests
    fn construction() {
        let x = BitSet::<usize, 1, 0, 4>::new();
        assert_eq!(x.len(), 0);
    }
}
