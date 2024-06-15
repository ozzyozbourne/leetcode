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

#[cfg(test)]
mod lc_120 {
    fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; triangle.len() + 1];
        for row in triangle.into_iter().rev() {
            for (i, v) in row.into_iter().enumerate() {
                dp[i] = v + std::cmp::min(dp[i], dp[i + 1]);
            }
        }
        dp[0]
    }

    #[test]
    fn lc_120_tests() {
        struct TestValue {
            input: Vec<Vec<i32>>,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input: vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]],
                expected: 11,
            },
            TestValue {
                input: vec![vec![-10]],
                expected: -10,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(minimum_total(t.input), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_611 {
    fn triangle_number(mut triangle: Vec<i32>) -> i32 {
        triangle.sort_unstable();
        let mut res = 0;
        for a in (1..triangle.len()).rev() {
            let (mut b, mut c) = (0, a - 1);
            while b < c {
                if triangle[b] + triangle[c] > triangle[a] {
                    res += c - b;
                    c -= 1;
                } else {
                    b += 1;
                }
            }
        }
        res as i32
    }

    #[test]
    fn lc_611_tests() {
        struct TestValue {
            input: Vec<i32>,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input: vec![2, 2, 3, 4],
                expected: 3,
            },
            TestValue {
                input: vec![4, 2, 3, 4],
                expected: 4,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(triangle_number(t.input), t.expected);
        }
    }
}
