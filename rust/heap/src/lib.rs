#[cfg(test)]
mod lc_2099 {

    fn max_subsequence(nums: Vec<i32>, r: i32) -> Vec<i32> {
        use std::{cmp::Reverse, collections::BinaryHeap};
        let mut heap = BinaryHeap::new();
        for (i, v) in nums.into_iter().enumerate() {
            heap.push(Reverse((v, i)));
            if heap.len() > r as usize {
                _ = heap.pop();
            }
        }
        let mut res = (0..r).map(|_| heap.pop().unwrap().0).collect::<Vec<_>>();
        res.sort_by_key(|&(i, _)| i);
        res.into_iter().map(|(x, _)| x).collect()
    }

    #[test]
    fn lc_509_tests() {
        struct TestValue {
            input1: Vec<i32>,
            input2: i32,
            expected: Vec<i32>,
        }

        let test_cases = [
            TestValue {
                input1: vec![2, 1, 3, 3],
                input2: 2,
                expected: vec![3, 3],
            },
            TestValue {
                input1: vec![-1, -2, 3, 4],
                input2: 3,
                expected: vec![-1, 3, 4],
            },
            TestValue {
                input1: vec![3, 4, 3, 3],
                input2: 2,
                expected: vec![3, 4],
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(max_subsequence(t.input1, t.input2), t.expected);
        }
    }
}

