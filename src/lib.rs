use std::{
    ops::{BitAnd, BitOr, BitXor, Sub},
    panic::{RefUnwindSafe, UnwindSafe},
};

type InputType = usize;
type BackingType = u64;
const BIT_WIDTH: InputType = BackingType::BITS as InputType;

// TODO:
// bench this:
//   - against IntSet
//   - against an impl with a vec as a backing type

#[derive(Debug, Clone)]
pub struct BitSet<const MIN: usize, const MAX: usize> {
    data: Vec<BackingType>,
    len: usize,
}

impl<const LOWER: usize, const UPPER: usize> Default for BitSet<LOWER, UPPER> {
    fn default() -> Self {
        Self {
            data: vec![0; (UPPER - LOWER).div_ceil(BIT_WIDTH) + 1],
            len: 0,
        }
    }
}

impl<const LOWER: usize, const UPPER: usize> BitSet<LOWER, UPPER> {
    /// Construct a new BitSet<LOWER, UPPER>, where `LOWER` and `UPPER` are
    /// `usize` integers that denote the boundaries of the BitSet.
    ///
    /// ```
    /// use fibis::BitSet;
    /// let foo = BitSet::<10, 20>::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of elements in the set.
    ///
    /// ```
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<0, 32>::new();
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
        debug_assert_eq!(
            self.data.iter().map(|x| x.count_ones()).sum::<u32>(),
            self.len as u32
        );
        self.len
    }

    /// Checks whether there are any elements present in the set.
    ///
    /// ```
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<0, 32>::new();
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
    /// let mut foo = BitSet::<0, 32>::new();
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
        // TODO: can I make sure this is vectorized?
        for x in self.data.iter_mut() {
            *x = 0_u64;
        }
        debug_assert_eq!(self.data.iter().map(|x| x.count_ones()).sum::<u32>(), 0);
        self.len = 0;
    }

    const LOWER_DIV_BIT_WIDTH: usize = LOWER / BIT_WIDTH;
    const LOWER_REM_BIT_WIDTH: usize = LOWER % BIT_WIDTH;

    /// Returns the array index and bit position for an element x.
    fn position(x: usize) -> (usize, usize) {
        (
            x / BIT_WIDTH - Self::LOWER_DIV_BIT_WIDTH,
            x % BIT_WIDTH - Self::LOWER_REM_BIT_WIDTH,
        )
    }

    /// Return whether an item is part of the set.
    ///
    /// ```
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<3, 32>::new();
    /// foo.insert(3);
    /// foo.insert(10);
    /// foo.insert(5);
    ///
    /// assert_eq!(foo.len(), 3);
    /// assert!(foo.contains(3));
    /// assert!(foo.contains(10));
    /// assert!(foo.contains(5));
    /// ```
    pub fn contains(&self, x: usize) -> bool {
        let (idx, bit) = Self::position(x);
        self.is_bit_set(idx, bit)
    }

    fn is_bit_set(&self, idx: usize, bit: usize) -> bool {
        self.data[idx] & (1 << bit) != 0
    }

    /// Insert an item into the set.
    ///
    /// This item `x` has to be constrained to `LOWER <= x <= UPPER`
    ///
    /// ```
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<0, 32>::new();
    /// foo.insert(0);
    /// foo.insert(10);
    /// foo.insert(32);
    /// assert!(foo.contains(0));
    /// assert!(foo.contains(10));
    /// assert!(foo.contains(32));
    /// assert!(!foo.contains(2));
    ///
    /// foo.remove(0);
    /// foo.remove(10);
    /// foo.remove(32);
    /// assert!(!foo.contains(0));
    /// assert!(!foo.contains(10));
    /// assert!(!foo.contains(32));
    /// ```
    pub fn insert(&mut self, x: usize) {
        assert!(x >= LOWER);
        assert!(x <= UPPER);
        let (idx, bit) = Self::position(x);
        //dbg!(x);
        //dbg!(idx);
        //dbg!(bit);
        //panic!("foo");
        self.data[idx] |= 1 << bit;
        self.len += 1;
    }

    /// Removes an item from the set.
    ///
    /// ```
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<0, 32>::new();
    /// foo.insert(1);
    /// assert!(foo.contains(1));
    ///
    /// foo.remove(1);
    /// assert!(!foo.contains(1));
    /// ```
    pub fn remove(&mut self, x: usize) {
        let (idx, bit) = Self::position(x);
        self.len = self.len.saturating_sub(self.is_bit_set(idx, bit).into());
        self.data[idx] ^= 1 << bit;
    }

    /// Using the predicate `f` passed to this method, filter the set such that
    /// it only retains elements fulfilling the predicate.
    ///
    /// ```
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<0, 32>::new();
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

    pub fn iter(&self) -> Iter<'_, LOWER, UPPER> {
        Iter {
            set: self,
            index: 0,
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

    /// Returns true of `self` and `other` do not share any elements.
    ///
    /// ```should_panic
    /// use fibis::BitSet;
    ///
    /// let mut foo = BitSet::<0, 32>::new();
    /// foo.insert(1);
    /// foo.insert(2);
    /// foo.insert(3);
    ///
    /// let mut bar = BitSet::<0, 32>::new();
    /// bar.insert(4);
    /// bar.insert(5);
    /// bar.insert(6);
    ///
    /// assert!(foo.is_disjoint(&bar));
    ///
    /// foo.insert(4);
    /// assert!(!foo.is_disjoint(&bar));
    /// ```
    pub fn is_disjoint(&self, other: &Self) -> bool {
        for x in self.into_iter() {
            if other.contains(x) {
                return false;
            }
        }
        true
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for x in self.into_iter() {
            if !other.contains(x) {
                return false;
            }
        }
        true
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }
}

impl<const LOWER: usize, const UPPER: usize> BitAnd for BitSet<LOWER, UPPER> {
    type Output = Self;
    fn bitand(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const LOWER: usize, const UPPER: usize> BitOr for BitSet<LOWER, UPPER> {
    type Output = Self;
    fn bitor(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const LOWER: usize, const UPPER: usize> BitXor for BitSet<LOWER, UPPER> {
    type Output = Self;
    fn bitxor(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl<const LOWER: usize, const UPPER: usize> Extend<InputType> for BitSet<LOWER, UPPER> {
    fn extend<I: IntoIterator<Item = InputType>>(&mut self, _iter: I) {
        todo!()
    }
}

impl<const LOWER: usize, const UPPER: usize, const M: usize> From<[InputType; M]>
    for BitSet<LOWER, UPPER>
{
    fn from(value: [InputType; M]) -> Self {
        Self::from_iter(value)
    }
}

impl<const LOWER: usize, const UPPER: usize> FromIterator<InputType> for BitSet<LOWER, UPPER> {
    fn from_iter<I: IntoIterator<Item = InputType>>(iter: I) -> Self {
        let mut s = Self::new();
        for x in iter {
            s.insert(x);
        }
        s
    }
}

impl<const LOWER: usize, const UPPER: usize> IntoIterator for BitSet<LOWER, UPPER> {
    type Item = InputType;
    type IntoIter = IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<const LOWER: usize, const UPPER: usize> IntoIterator for &BitSet<LOWER, UPPER> {
    type Item = InputType;
    type IntoIter = IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

impl<const LOWER: usize, const UPPER: usize> PartialEq for BitSet<LOWER, UPPER> {
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

impl<const LOWER: usize, const UPPER: usize> Eq for &BitSet<LOWER, UPPER> {}

impl<const LOWER: usize, const UPPER: usize> Sub for &BitSet<LOWER, UPPER> {
    type Output = Self;
    fn sub(self, _rhs: Self) -> Self::Output {
        // basically just the difference but as a new Set
        todo!()
    }
}

unsafe impl<const LOWER: usize, const UPPER: usize> Send for BitSet<LOWER, UPPER> {}
unsafe impl<const LOWER: usize, const UPPER: usize> Sync for BitSet<LOWER, UPPER> {}
impl<const LOWER: usize, const UPPER: usize> RefUnwindSafe for BitSet<LOWER, UPPER> {}
impl<const LOWER: usize, const UPPER: usize> UnwindSafe for BitSet<LOWER, UPPER> {}

pub struct Difference {}
pub struct SymmetricDifference {}
pub struct Intersection {}
pub struct Union {}
pub struct IntoIter {}
pub struct Iter<'a, const LOWER: usize, const UPPER: usize> {
    set: &'a BitSet<LOWER, UPPER>,
    index: usize,
}

impl<'a, const LOWER: usize, const UPPER: usize> Iterator for Iter<'a, LOWER, UPPER> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.set.len {
            self.index += 1;
            Some(self.index + LOWER)
        } else {
            None
        }
    }
}

impl Iterator for IntoIter {
    type Item = InputType;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
