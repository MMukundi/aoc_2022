use std::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    marker::PhantomData,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, IndexMut, Not},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DefaultedBytes<const N: usize, const DEFAULT_BYTE: u8 = 0>([u8; N]);
impl<const N: usize, const DEFAULT_BYTE: u8> Default for DefaultedBytes<N, DEFAULT_BYTE> {
    fn default() -> Self {
        Self([DEFAULT_BYTE; N])
    }
}
impl<const N: usize, const DEFAULT_BYTE: u8> FromIterator<u8> for DefaultedBytes<N, DEFAULT_BYTE> {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut bytes = Self::default();
        for (byte, mut_byte) in iter.into_iter().zip(bytes.iter_mut()) {
            *mut_byte = byte;
        }
        bytes
    }
}
impl<const N: usize, const DEFAULT_BYTE: u8> IntoIterator for DefaultedBytes<N, DEFAULT_BYTE> {
    type IntoIter = std::array::IntoIter<u8, N>;
    type Item = u8;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a, const N: usize, const DEFAULT_BYTE: u8> IntoIterator
    for &'a DefaultedBytes<N, DEFAULT_BYTE>
{
    type IntoIter = std::slice::Iter<'a, u8>;
    type Item = &'a u8;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
impl<'a, const N: usize, const DEFAULT_BYTE: u8> IntoIterator
    for &'a mut DefaultedBytes<N, DEFAULT_BYTE>
{
    type IntoIter = std::slice::IterMut<'a, u8>;
    type Item = &'a mut u8;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
impl<const N: usize, const DEFAULT_BYTE: u8> Index<usize> for DefaultedBytes<N, DEFAULT_BYTE> {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}
impl<const N: usize, const DEFAULT_BYTE: u8> IndexMut<usize> for DefaultedBytes<N, DEFAULT_BYTE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<const N: usize, const DEFAULT_BYTE: u8> DefaultedBytes<N, DEFAULT_BYTE> {
    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<u8> {
        self.0.iter_mut()
    }
}

#[derive(Hash)]
pub struct BitSet<T=usize, B = Vec<u8>> {
    bytes: B,
    phantom_elements: PhantomData<T>,
}

impl<T: Into<usize> + Debug, B: IndexMut<usize, Output = u8>> Extend<T> for BitSet<T, B>
where
    Self: Debug,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            // dbg!(&item);
            // dbg!(&self);
            self.insert(item);
        }
    }
}
impl<T, B: PartialEq> PartialEq for BitSet<T, B> {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}
impl<T, B: Eq> Eq for BitSet<T, B> {}
impl<T, B: Copy> Copy for BitSet<T, B> {}
impl<T, B: Clone> Clone for BitSet<T, B> {
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
            phantom_elements: PhantomData,
        }
    }
}
impl<T, B: Debug> Debug for BitSet<T, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitSet")
            .field("bytes", &self.bytes)
            .finish()
    }
}
impl<T> BitSet<T, Vec<u8>> {
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: vec![0; (capacity + 7) & (!7)],
            phantom_elements: PhantomData,
        }
    }
}
impl<T: Into<usize>> BitSet<T, Vec<u8>> {
    #[inline]
    pub fn with_max(value: T) -> Self {
        Self::with_capacity(value.into() + 1)
    }
}
impl<T, B: Default> Default for BitSet<T, B> {
    fn default() -> Self {
        Self {
            bytes: Default::default(),
            phantom_elements: PhantomData,
        }
    }
}

pub struct BitSetIndex {
    byte_index: usize,
    bitmask: u8,
}

#[inline]
fn to_bitset_index(index: usize) -> BitSetIndex {
    let div = index / 8;
    let rem = index % 8;
    BitSetIndex {
        byte_index: div,
        bitmask: 1u8 << rem,
    }
}

impl<T: Into<usize>, B: IndexMut<usize, Output = u8>> BitSet<T, B> {
    pub fn contains(&self, value: T) -> bool {
        let index = to_bitset_index(value.into());
        (self.bytes[index.byte_index] & index.bitmask) != 0
    }

    // #[cfg(test)]
    // pub fn toggle(&mut self,value:T)->bool {
    //     let index = to_bitset_index(value.into());

    //     let byte = &mut self.bytes[index.byte_index];
    //     let byte_value = *byte;

    //     let contained = (byte_value & index.bitmask) != 0;
    //     *byte = byte_value ^ index.bitmask;
    //     contained
    // }

    pub fn insert(&mut self, value: T) -> bool {
        let index = to_bitset_index(value.into());

        let byte = &mut self.bytes[index.byte_index];
        let byte_value = *byte;

        let contained = (byte_value & index.bitmask) != 0;
        *byte = byte_value | index.bitmask;
        contained
    }
    pub fn remove(&mut self, value: T) -> bool {
        let index = to_bitset_index(value.into());

        let byte = &mut self.bytes[index.byte_index];
        let byte_value = *byte;

        let contained = (byte_value & index.bitmask) != 0;
        *byte = byte_value & (!index.bitmask);
        contained
    }
}
impl<T, B> BitSet<T, B>
where
    for<'a> &'a B: IntoIterator,
    for<'a> <&'a B as IntoIterator>::Item: Borrow<u8>,
{
    pub fn iter(&self) -> BitsetIterator<T, <&B as IntoIterator>::IntoIter> {
        BitsetIterator::new(&self.bytes)
    }
    pub fn is_empty(&self) -> bool {
        self.bytes.borrow().into_iter().all(|b|*b.borrow() == 0)
    }
}
impl<T, B> BitSet<T, B>
where
    for<'a> &'a mut B: IntoIterator,
    for<'a> <&'a mut B as IntoIterator>::Item: BorrowMut<u8>,
{
    pub fn clear(&mut self) {
        self.bytes.borrow_mut().into_iter().for_each(|mut b|*b.borrow_mut()=0);
    }
}
impl<T, B> BitAndAssign for BitSet<T, B>
where
    for<'a> &'a B: IntoIterator,
    for<'a> <&'a B as IntoIterator>::Item: Borrow<u8>,
    for<'a> &'a mut B: IntoIterator,
    for<'a> <&'a mut B as IntoIterator>::Item: BorrowMut<u8>,
{
    fn bitand_assign(&mut self, rhs: Self) {
        for (mut a, b) in
            IntoIterator::into_iter(&mut self.bytes).zip(IntoIterator::into_iter(&rhs.bytes))
        {
            *a.borrow_mut() &= *b.borrow();
        }
    }
}
impl<T, B> BitOrAssign for BitSet<T, B>
where
    for<'a> &'a B: IntoIterator,
    for<'a> <&'a B as IntoIterator>::Item: Borrow<u8>,
    for<'a> &'a mut B: IntoIterator,
    for<'a> <&'a mut B as IntoIterator>::Item: BorrowMut<u8>,
{
    fn bitor_assign(&mut self, rhs: Self) {
        for (mut a, b) in
            IntoIterator::into_iter(&mut self.bytes).zip(IntoIterator::into_iter(&rhs.bytes))
        {
            *a.borrow_mut() |= *b.borrow();
        }
    }
}
impl<T, B> BitXorAssign for BitSet<T, B>
where
    for<'a> &'a B: IntoIterator,
    for<'a> <&'a B as IntoIterator>::Item: Borrow<u8>,
    for<'a> &'a mut B: IntoIterator,
    for<'a> <&'a mut B as IntoIterator>::Item: BorrowMut<u8>,
{
    fn bitxor_assign(&mut self, rhs: Self) {
        for (mut a, b) in
            IntoIterator::into_iter(&mut self.bytes).zip(IntoIterator::into_iter(&rhs.bytes))
        {
            *a.borrow_mut() ^= *b.borrow();
        }
    }
}
impl<T, B> BitAnd for BitSet<T, B>
where
    for<'a> &'a B: IntoIterator,
    for<'a> <&'a B as IntoIterator>::Item: Borrow<u8>,
    for<'a> &'a mut B: IntoIterator,
    for<'a> <&'a mut B as IntoIterator>::Item: BorrowMut<u8>,
{
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl<T, B> BitOr for BitSet<T, B>
where
    for<'a> &'a B: IntoIterator,
    for<'a> <&'a B as IntoIterator>::Item: Borrow<u8>,
    for<'a> &'a mut B: IntoIterator,
    for<'a> <&'a mut B as IntoIterator>::Item: BorrowMut<u8>,
{
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl<T, B> BitXor for BitSet<T, B>
where
    for<'a> &'a B: IntoIterator,
    for<'a> <&'a B as IntoIterator>::Item: Borrow<u8>,
    for<'a> &'a mut B: IntoIterator,
    for<'a> <&'a mut B as IntoIterator>::Item: BorrowMut<u8>,
{
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl<T, B> Not for BitSet<T, B>
where
    for<'a> &'a mut B: IntoIterator,
    for<'a> <&'a mut B as IntoIterator>::Item: BorrowMut<u8>,
{
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for mut byte in &mut self.bytes {
            let mut_bute = byte.borrow_mut();
            *mut_bute = !*mut_bute;
        }
        self
    }
}
impl<T: TryFrom<usize>, B: IntoIterator<Item = u8>> IntoIterator for BitSet<T, B>
where
    B::Item: Borrow<u8>,
{
    type Item = Result<T, T::Error>;

    type IntoIter = BitsetIterator<T, B::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        let mut byte_iter = self.bytes.into_iter();
        BitsetIterator {
            current_byte: byte_iter.next(),
            phantom_data: PhantomData,
            byte_iter,
            bits_consumed: 0,
        }
    }
}

pub struct BitsetIterator<T, I> {
    current_byte: Option<u8>,
    bits_consumed: usize,
    byte_iter: I,
    phantom_data: PhantomData<T>,
}
impl<T, I: Iterator> BitsetIterator<T, I>
where
    I::Item: Borrow<u8>,
{
    pub fn new<II: IntoIterator<IntoIter = I>>(iter: II) -> Self {
        let mut byte_iter = iter.into_iter();
        BitsetIterator {
            current_byte: byte_iter.next().map(|b| *b.borrow()),
            phantom_data: PhantomData,
            byte_iter,
            bits_consumed: 0,
        }
    }
}
impl<T, I> Debug for BitsetIterator<T, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BitsetIterator")
            .field("current_byte", &self.current_byte)
            .field("bits_consumed", &self.bits_consumed)
            .finish()
    }
}

#[inline]
pub fn unborrow_byte<B: Borrow<u8>>(byte: B) -> u8 {
    *byte.borrow()
    // let byte = *byte.borrow();
    // println!("Read byte: {:#b}",byte);
    // byte
}

impl<T: TryFrom<usize>, I: Iterator> Iterator for BitsetIterator<T, I>
where
    I::Item: Borrow<u8>,
{
    type Item = Result<T, T::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current_byte = self.current_byte?;
            let trailing = current_byte.trailing_zeros() as u8;
            // dbg!(&self,trailing);
            // If the new index isn't a multiple of 8, (only partially into the current byte)
            // Or the offset is 0 (current bit is set)
            if trailing == 8 {
                self.current_byte = self.byte_iter.next().map(unborrow_byte); // Read next byte
                self.bits_consumed = (self.bits_consumed & !0b111usize) + 0b1000usize;
            // Next multiple of 8 because next byte
            } else {
                let new_index = trailing as usize + self.bits_consumed;
                self.bits_consumed = new_index;
                if trailing == 0 || new_index & 0b111usize != 0 {
                    let next_byte = current_byte
                        .checked_shr((trailing + 1) as u32)
                        .unwrap_or_default(); // Consume the (presumably set) bit
                                              // println!("Next byte: {:#b}",next_byte);
                    if next_byte == 0 {
                        self.current_byte = self.byte_iter.next().map(unborrow_byte); // Read next byte
                        self.bits_consumed = (self.bits_consumed & !0b111usize) + 0b1000usize;
                    // Next multiple of 8 because next byte
                    // dbg!(&self.current_byte);
                    } else {
                        self.bits_consumed += 1;
                        self.current_byte = Some(next_byte);
                    }
                    // self.current_byte = Some(next_byte);

                    return Some(T::try_from(new_index));
                } else {
                    self.current_byte = self.byte_iter.next().map(unborrow_byte);
                    // Read next byte
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[inline]
    pub fn unwrap_infallible<T>(result: Result<T, Infallible>) -> T {
        match result {
            Ok(value) => value,
            Err(infallible_error) => match infallible_error {},
        }
    }
    use std::convert::Infallible;

    use super::BitSet;

    const PRIMES: [usize; 25] = [
        2, 3, 5, 7, 9, 11, 13, 17, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 91,
        97,
    ];
    #[test]
    pub fn test_iter() {
        let mut bitset = BitSet::<usize>::with_max(100);
        for p in PRIMES {
            bitset.insert(p);
        }
        for (bitset_prime, const_prime) in bitset
            .iter()
            .zip(PRIMES.into_iter())
            .map(|(bitset_prime, prime)| (unwrap_infallible(bitset_prime), prime))
        {
            assert_eq!(bitset_prime, const_prime);
        }
    }
    #[test]
    pub fn test_into_iter() {
        let mut bitset = BitSet::<usize>::with_max(100);
        for p in PRIMES {
            bitset.insert(p);
        }
        for (bitset_prime, const_prime) in bitset.into_iter().zip(PRIMES.into_iter()) {
            if let Ok(bitset_prime) = bitset_prime {
                dbg!(bitset_prime, const_prime);
                assert_eq!(bitset_prime, const_prime)
            }
        }
    }

    #[test]
    pub fn test_duplicate_insert() {
        let mut bitset = BitSet::<usize>::with_max(100);
        assert!(!bitset.insert(2));
        assert!(bitset.insert(2));
    }

    #[test]
    pub fn test_duplicate_remove() {
        let mut bitset = BitSet::<usize>::with_max(100);
        assert!(!bitset.remove(2));
        bitset.insert(2);
        assert!(bitset.remove(2));
        assert!(!bitset.remove(2));
    }

    #[test]
    pub fn test_contains() {
        let mut bitset = BitSet::<usize>::with_max(100);
        assert!(!bitset.contains(2));
        bitset.insert(2);
        assert!(bitset.contains(2));
        bitset.remove(2);
        assert!(!bitset.contains(2));
    }
}
