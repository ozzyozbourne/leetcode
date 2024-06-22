#[cfg(test)]
mod lc_1052 {
    fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {}

    #[test]
    fn tests() {
        struct TestValue {
            input1: Vec<i32>,
            input2: Vec<i32>,
            input3: i32,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input1: vec![1, 0, 1, 2, 1, 1, 7, 5],
                input2: vec![0, 1, 0, 1, 0, 1, 0, 1],
                input3: 3,
                expected: 16,
            },
            TestValue {
                input1: vec![1],
                input2: vec![0],
                input3: 1,
                expected: 1,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(max_satisfied(t.input1, t.input2, t.input3), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_1248 {
    fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let (mut l, mut m, mut odd, mut res) = (0, 0, 0, 0);
        for r in 0..nums.len() {
            if nums[r] & 1 == 1 {
                odd += 1;
            }
            while odd > k {
                if nums[l] & 1 == 1 {
                    odd -= 1;
                }
                l += 1;
                m = l;
            }
            if odd == k {
                while nums[m] & 1 != 1 {
                    m += 1;
                }
            }
            res += m - l + 1
        }
        res as i32
    }

    #[test]
    fn tests() {
        struct TestValue {
            input1: Vec<i32>,
            input2: i32,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input1: vec![2, 2, 2, 1, 2, 2, 2, 1, 2],
                input2: 2,
                expected: 8,
            },
            TestValue {
                input1: vec![2, 4, 6],
                input2: 1,
                expected: 0,
            },
            TestValue {
                input1: vec![1, 1, 2, 1, 1],
                input2: 3,
                expected: 2,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(number_of_subarrays(t.input1, t.input2), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_930 {
    fn num_subarrays_with_sum(nums: Vec<i32>, k: i32) -> i32 {}

    #[test]
    fn tests() {
        struct TestValue {
            input1: Vec<i32>,
            input2: i32,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input1: vec![1, 0, 1, 0, 1],
                input2: 2,
                expected: 4,
            },
            TestValue {
                input1: vec![0, 0, 0, 0, 0],
                input2: 0,
                expected: 15,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(num_subarrays_with_sum(t.input1, t.input2), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_560 {
    fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {}

    #[test]
    fn tests() {
        struct TestValue {
            input1: Vec<i32>,
            input2: i32,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input1: vec![1, 1, 1],
                input2: 2,
                expected: 2,
            },
            TestValue {
                input1: vec![1, 2, 3],
                input2: 3,
                expected: 2,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(subarray_sum(t.input1, t.input2), t.expected);
        }
    }
}
