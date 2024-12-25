use std::{
    ops::{BitAnd, BitOr, BitXor, Sub},
    panic::{RefUnwindSafe, UnwindSafe},
};

use num_integer::Integer;

type InputType = usize;
type BackingType = u64;
const BIT_WIDTH: InputType = BackingType::BITS as InputType;

#[derive(Debug, Clone)]
pub struct BitSet<const N: usize, const MIN: usize, const MAX: usize> {
    data: [BackingType; N],
    len: usize,
}

impl<const N: usize, const LOWER: usize, const UPPER: usize> Default for BitSet<N, LOWER, UPPER> {
    fn default() -> Self {
        assert_eq!(
            N,
            (UPPER - LOWER).div_ceil(BIT_WIDTH),
            "fibis::BitSet size N must equal (UPPER - LOWER).div_ceil({})",
            BIT_WIDTH
        );
        Self {
            data: [0_u64; N],
            len: 0,
        }
    }
}

impl<const N: usize, const LOWER: usize, const UPPER: usize> BitSet<N, LOWER, UPPER> {
    const _N_FITS: () = assert!(N == ((UPPER - LOWER).div_ceil(BIT_WIDTH)));

    /// Construct a new BitSet<N, LOWER, UPPER>, where `LOWER` and `UPPER` are
    /// `usize` integers that denote the boundaries of the BitSet. `N` is the
    /// size of the backing array of the set and, at compile time, it is
    /// asserted that
    /// `N == ((UPPER - LOWER).div_ceil(64))`
    ///
    /// ```
    /// use fibis::BitSet;
    /// let foo = BitSet::<1, 10, 20>::new(); // (20 - 10).div_ceil(64) == 1
    /// ```
    ///
    /// ```should_panic
    /// use fibis::BitSet;
    ///
    /// // panics, because (512 - 5).div_ceil(64) != 1
    /// let bar = BitSet::<1, 5, 512>::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of elements in the set.
    ///
    /// ```
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<1, 0, 32>::new();
    /// foo.insert(1);
    /// foo.insert(10);
    /// foo.insert(5);
    ///
    /// assert_eq!(foo.len(), 3);
    ///
    /// foo.remove(1);
    /// assert_eq!(foo.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks whether there are any elements present in the set.
    ///
    /// ```
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<1, 0, 32>::new();
    /// foo.insert(1);
    /// assert_eq!(foo.len(), 1);
    /// assert!(!foo.is_empty());
    ///
    /// foo.remove(1);
    /// assert!(foo.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Removes all items from set.
    ///
    /// ```
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<1, 0, 32>::new();
    /// foo.insert(1);
    /// foo.insert(10);
    /// foo.insert(5);
    ///
    /// assert_eq!(foo.len(), 3);
    /// assert!(!foo.is_empty());
    ///
    /// foo.clear();
    /// assert_eq!(foo.len(), 0);
    /// assert!(foo.is_empty());
    /// ```
    pub fn clear(&mut self) {
        for x in self.data.iter_mut() {
            *x = 0_u64;
        }
        self.len = 0;
    }

    /// Returns the array index and bit position for an element x.
    fn position(x: usize) -> (usize, usize) {
        x.div_mod_floor(&BIT_WIDTH)
    }

    /// Return whether an item is part of the set.
    ///
    /// ```
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<1, 0, 32>::new();
    /// foo.insert(1);
    /// foo.insert(10);
    /// foo.insert(5);
    ///
    /// assert_eq!(foo.len(), 3);
    /// assert!(foo.contains(1));
    /// assert!(foo.contains(10));
    /// assert!(foo.contains(5));
    /// ```
    pub fn contains(&self, x: usize) -> bool {
        let (idx, bit) = Self::position(x);
        self.data[idx] & (1 << bit) != 0
    }

    /// Insert an item into the set.
    ///
    /// This item `x` has to be constrained to `LOWER <= x <= UPPER`
    ///
    /// ```
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<1, 0, 32>::new();
    /// foo.insert(1);
    /// assert!(foo.contains(1));
    /// assert!(!foo.contains(2));
    /// foo.remove(1);
    /// assert!(!foo.contains(1));
    /// ```
    pub fn insert(&mut self, x: usize) {
        assert!(x >= LOWER);
        assert!(x <= UPPER);
        let (idx, bit) = Self::position(x);
        self.data[idx] |= 1 << bit;
        self.len += 1;
    }

    /// Removes an item from the set.
    ///
    /// ```
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<1, 0, 32>::new();
    /// foo.insert(1);
    /// assert!(foo.contains(1));
    ///
    /// foo.remove(1);
    /// assert!(!foo.contains(1));
    /// ```
    pub fn remove(&mut self, x: usize) {
        let (idx, bit) = Self::position(x);
        self.data[idx] ^= 1 << bit;
        self.len = self.len.saturating_sub(1);
    }

    /// Using the predicate `f` passed to this method, filter the set such that
    /// it only retains elements fulfilling the predicate.
    ///
    /// ```
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<1, 0, 32>::new();
    /// foo.insert(1);
    /// foo.insert(2);
    /// foo.insert(3);
    ///
    /// foo.retain(|x| x % 2 == 0);
    ///
    /// assert!(foo.contains(2));
    /// assert!(!foo.contains(1));
    /// assert!(!foo.contains(3));
    /// ```
    pub fn retain(&mut self, f: impl Fn(usize) -> bool) {
        for x in LOWER..=UPPER {
            if !f(x) {
                self.remove(x);
            }
        }
    }

    /// Visits the values representing the difference, i.e., the values that are in self but not in other.
    pub fn difference(&self, _other: &Self) -> Difference {
        todo!()
    }

    /// Visits the values representing the symmetric difference, i.e., the values that are in self or in other but not in both.
    pub fn symmetric_difference(&self, _other: &Self) -> SymmetricDifference {
        todo!()
    }

    /// Visits the values representing the intersection, i.e., the values that are both in self and other.
    ///
    /// When an equal element is present in self and other then the resulting Intersection may yield references to one or the other. This can be relevant if T contains fields which are not compared by its Eq implementation, and may hold different value between the two equal copies of T in the two sets.
    pub fn intersection(&self, _other: &Self) -> Intersection {
        todo!()
    }

    /// Visits the values representing the union, i.e., all the values in self or other, without duplicates.
    pub fn union(&self, _other: &Self) -> Union {
        todo!()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        todo!()
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl<const N: usize, const LOWER: usize, const UPPER: usize> BitAnd for BitSet<N, LOWER, UPPER> {
    type Output = Self;
    fn bitand(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize, const LOWER: usize, const UPPER: usize> BitOr for BitSet<N, LOWER, UPPER> {
    type Output = Self;
    fn bitor(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize, const LOWER: usize, const UPPER: usize> BitXor for BitSet<N, LOWER, UPPER> {
    type Output = Self;
    fn bitxor(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const N: usize, const LOWER: usize, const UPPER: usize> Extend<InputType>
    for BitSet<N, LOWER, UPPER>
{
    fn extend<I: IntoIterator<Item = InputType>>(&mut self, _iter: I) {
        todo!()
    }
}

impl<const N: usize, const LOWER: usize, const UPPER: usize, const M: usize> From<[InputType; M]>
    for BitSet<N, LOWER, UPPER>
{
    fn from(_value: [InputType; M]) -> Self {
        todo!()
    }
}

impl<const N: usize, const LOWER: usize, const UPPER: usize> FromIterator<InputType>
    for BitSet<N, LOWER, UPPER>
{
    fn from_iter<I: IntoIterator<Item = InputType>>(iter: I) -> Self {
        let mut s = Self::new();
        for x in iter {
            s.insert(x);
        }
        s
    }
}

impl<const N: usize, const LOWER: usize, const UPPER: usize> IntoIterator
    for BitSet<N, LOWER, UPPER>
{
    type Item = InputType;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<const N: usize, const LOWER: usize, const UPPER: usize> IntoIterator
    for &BitSet<N, LOWER, UPPER>
{
    type Item = InputType;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<const N: usize, const LOWER: usize, const UPPER: usize> PartialEq for BitSet<N, LOWER, UPPER> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        for (x, y) in self.into_iter().zip(other) {
            if x != y {
                return false;
            }
        }
        true
    }
}

impl<const N: usize, const LOWER: usize, const UPPER: usize> Eq for &BitSet<N, LOWER, UPPER> {}

impl<const N: usize, const LOWER: usize, const UPPER: usize> Sub for &BitSet<N, LOWER, UPPER> {
    type Output = Self;
    fn sub(self, _rhs: Self) -> Self::Output {
        // basically just the difference but as a new Set
        todo!()
    }
}

unsafe impl<const N: usize, const LOWER: usize, const UPPER: usize> Send
    for BitSet<N, LOWER, UPPER>
{
}
unsafe impl<const N: usize, const LOWER: usize, const UPPER: usize> Sync
    for BitSet<N, LOWER, UPPER>
{
}
impl<const N: usize, const LOWER: usize, const UPPER: usize> RefUnwindSafe
    for BitSet<N, LOWER, UPPER>
{
}
impl<const N: usize, const LOWER: usize, const UPPER: usize> UnwindSafe
    for BitSet<N, LOWER, UPPER>
{
}

pub struct Difference {}
pub struct SymmetricDifference {}
pub struct Intersection {}
pub struct Union {}
pub struct IntoIter {}

impl Iterator for IntoIter {
    type Item = InputType;
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
        let x = BitSet::<1, 0, 10>::new();
        assert_eq!(x.len(), 0);
    }
}
