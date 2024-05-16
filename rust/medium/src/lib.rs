#[cfg(test)]
mod lc_137 {

    fn single_number(nums: Vec<i32>) -> i32 {
        let (mut ones, mut twos) = (0, 0);
        for n in nums {
            ones = (ones ^ n) & !twos;
            twos = (twos ^ n) & !ones;
        }
        ones
    }
    #[test]
    fn test_lc_137_one() {
        assert_eq!(single_number(vec![2, 2, 3, 2]), 3);
    }

    #[test]
    fn test_lc_137_two() {
        assert_eq!(single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
