#[cfg(test)]
mod ngr {

    use std::collections::VecDeque;

    fn ngr_mono(nums: Vec<i32>) -> Vec<i32> {
        let (mut st, mut res): (VecDeque<usize>, Vec<i32>) =
            (VecDeque::new(), vec![-1; nums.len()]);
        for (index, &value) in nums.iter().enumerate() {
            while !st.is_empty() && nums[*st.back().unwrap()] < value {
                res[st.pop_back().unwrap()] = value;
            }
            st.push_back(index);
        }
        res
    }

    #[test]
    fn test_ngr_mono_one() {
        assert_eq!(ngr_mono(vec![1, 3, 2, 4]), vec![3, 4, 4, -1]);
    }

    #[test]
    fn test_ngr_mono_two() {
        assert_eq!(ngr_mono(vec![5, 4, 3, 2, 1]), vec![-1, -1, -1, -1, -1]);
    }

    #[test]
    fn test_ngr_mono_three() {
        assert_eq!(ngr_mono(vec![20, 70, 30, 80, 60]), vec![70, 80, 80, -1, -1]);
    }
}
