trait Bitset {
    fn is_bit_on(&self, digit: usize) -> bool;
    fn to_bit_vec(&self, len: usize) -> Vec<bool>;
}

impl Bitset for usize {
    fn is_bit_on(&self, digit: usize) -> bool {
        self >> digit & 1 == 1
    }

    fn to_bit_vec(&self, len: usize) -> Vec<bool> {
        (0..len).map(|digit| self.is_bit_on(digit)).collect()
    }
}
