#[derive(Debug, Clone)]
pub struct BitSet<T, const Size: usize, const Min: usize, const Max: usize> {
    data: [T; Size],
    len: usize,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
