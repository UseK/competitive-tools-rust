pub trait Bitset {
    fn is_bit_on(&self, digit: usize) -> bool;
    fn to_bit_vec(&self, len: usize) -> Vec<bool>;
}

impl Bitset for usize {
    ///
    /// ```
    /// use competitive_tools_rust::bitset::Bitset;
    /// assert_eq!(5.is_bit_on(0), true);
    /// assert_eq!(5.is_bit_on(1), false);
    /// assert_eq!(5.is_bit_on(2), true);
    /// assert_eq!(5.is_bit_on(3), false);
    /// assert_eq!(5.is_bit_on(4), false);
    ///
    /// assert_eq!(2.is_bit_on(0), false);
    /// assert_eq!(2.is_bit_on(1), true);
    /// assert_eq!(2.is_bit_on(2), false);
    /// assert_eq!(5.is_bit_on(3), false);
    /// ```
    fn is_bit_on(&self, digit: usize) -> bool {
        self >> digit & 1 == 1
    }

    fn to_bit_vec(&self, len: usize) -> Vec<bool> {
        (0..len).map(|digit| self.is_bit_on(digit)).collect()
    }
}
