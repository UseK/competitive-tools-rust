trait Vectools {
    fn keep(&mut self, bools: &Vec<bool>);
}

impl<T> Vectools for Vec<T> {
    fn keep(&mut self, bools: &Vec<bool>) {
        let mut i = 0;
        self.retain(|_| (bools[i], i +=1).0)
    }
}

#[cfg(test)]
mod tests {
    use crate::vectools::Vectools;

    #[test]
    fn test() {
        let mut v = vec![1, 2, 3, 4, 5];
        let bools = vec![false, true, true, false, true];
        v.keep(&bools);
        assert_eq!(v, vec![2, 3, 5]);
    }
}