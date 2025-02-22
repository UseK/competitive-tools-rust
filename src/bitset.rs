pub trait Bitset {
    fn is_bit_on(&self, digit: usize) -> bool;
    fn to_bit_vec(&self, len: usize) -> Vec<bool>;
    fn toggled_bit(&self, digit: usize) -> Self;
    fn turned_bit_on(&self, digit: usize) -> Self;
    fn turned_bit_off(&self, digit: usize) -> Self;
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
        (self >> digit) & 1 == 1
    }

    fn to_bit_vec(&self, len: usize) -> Vec<bool> {
        (0..len).map(|digit| self.is_bit_on(digit)).collect()
    }

    /// ```
    /// use competitive_tools_rust::bitset::Bitset;
    /// // 5 is 0000101 in bit
    /// assert_eq!(5.toggled_bit(0), 4);
    /// assert_eq!(5.toggled_bit(1), 7);
    /// assert_eq!(5.toggled_bit(2), 1);
    /// assert_eq!(5.toggled_bit(3), 13);
    /// assert_eq!(5.toggled_bit(4), 21);
    /// for i in 0..20 {
    ///     assert!(0.toggled_bit(i).is_bit_on(i));
    /// }
    /// ```
    fn toggled_bit(&self, digit: usize) -> Self {
        *self ^ (1 << digit)
    }

    /// ```
    /// use competitive_tools_rust::bitset::Bitset;
    /// // 5 is 0000101 in bit
    /// assert_eq!(5.turned_bit_on(0), 5);
    /// assert_eq!(5.turned_bit_on(1), 7);
    /// assert_eq!(5.turned_bit_on(2), 5);
    /// assert_eq!(5.turned_bit_on(3), 13);
    /// assert_eq!(5.turned_bit_on(4), 21);
    /// for i in 0..20 {
    ///     assert!(0.turned_bit_on(i).is_bit_on(i));
    /// }
    /// ```
    fn turned_bit_on(&self, digit: usize) -> Self {
        if !self.is_bit_on(digit) {
            self.toggled_bit(digit)
        } else {
            *self
        }
    }

    /// ```
    /// use competitive_tools_rust::bitset::Bitset;
    /// // 5 is 0000101 in bit
    /// assert_eq!(5.turned_bit_off(0), 4);
    /// assert_eq!(5.turned_bit_off(1), 5);
    /// assert_eq!(5.turned_bit_off(2), 1);
    /// assert_eq!(5.turned_bit_off(3), 5);
    /// assert_eq!(5.turned_bit_off(4), 5);
    /// for i in 0..20 {
    ///     assert!(!127.turned_bit_off(i).is_bit_on(i));
    /// }
    /// ```
    fn turned_bit_off(&self, digit: usize) -> Self {
        if self.is_bit_on(digit) {
            self.toggled_bit(digit)
        } else {
            *self
        }
    }
}
