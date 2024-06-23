#[cfg(test)]
mod lc_509 {
    fn tail_recursive_factorial(n: i32, r: i32) -> i32 {
        if n <= 1 {
            return r;
        }
        tail_recursive_factorial(n - 1, n * r)
    }

    #[test]
    fn lc_509_tests() {
        struct TestValue {
            input1: i32,
            input2: i32,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input1: 5,
                input2: 1,
                expected: 120,
            },
            TestValue {
                input1: 4,
                input2: 1,
                expected: 24,
            },
            TestValue {
                input1: 3,
                input2: 1,
                expected: 6,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(tail_recursive_factorial(t.input1, t.input2), t.expected);
        }
    }
}

#[cfg(test)]
mod no_lc {
    fn all_subsequences(n: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::<Vec<i32>>::new();
        rec(&mut res, &mut Vec::new(), &n, 0);
        res
    }

    fn rec(res: &mut Vec<Vec<i32>>, acc: &mut Vec<i32>, input: &Vec<i32>, i: usize) {
        if i >= input.len() {
            res.push(acc.clone());
            return;
        }
        acc.push(input[i]);
        rec(res, acc, input, i + 1);
        _ = acc.pop();
        rec(res, acc, input, i + 1);
    }

    #[test]
    fn no_lc_tests() {
        struct TestValue {
            input: Vec<i32>,
            expected: Vec<Vec<i32>>,
        }

        let test_cases = [
            TestValue {
                input: vec![1, 2, 3],
                expected: vec![
                    vec![],
                    vec![1, 2, 3],
                    vec![1],
                    vec![2],
                    vec![3],
                    vec![1, 2],
                    vec![1, 3],
                    vec![2, 3],
                ],
            },
            TestValue {
                input: vec![1, 2],
                expected: vec![vec![], vec![1, 2], vec![1], vec![2]],
            },
            TestValue {
                input: vec![1],
                expected: vec![vec![], vec![1]],
            },
        ];

        for t in test_cases.into_iter() {
            let res = all_subsequences(t.input);
            assert_eq!(res.len(), t.expected.len());
            for v in res.into_iter() {
                assert!(t.expected.contains(&v));
            }
        }
    }
}

#[cfg(test)]
mod lc_9 {
    fn is_palindrome(x: i32) -> bool {
        fn reverse(x: i32, r: i32) -> i32 {
            match (x, r) {
                (_, _) if x < 10 => r * 10 + x,
                _ => reverse(x / 10, r * 10 + x % 10),
            }
        }
        match x {
            _ if x < 0 => false,
            _ if x < 10 => true,
            _ => reverse(x, 0) == x,
        }
    }

    #[test]
    fn lc_9_tests() {
        struct TestValue {
            input: i32,
            expected: bool,
        }

        let test_cases = [
            TestValue {
                input: 121,
                expected: true,
            },
            TestValue {
                input: -121,
                expected: false,
            },
            TestValue {
                input: 10,
                expected: false,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(is_palindrome(t.input), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_7 {
    fn reverse(mut x: i32) -> i32 {
        let mut rev = 0;
        while x != 0 {
            let pop = x % 10;
            x /= 10;
            if rev > i32::MAX / 10 || (rev == i32::MAX / 10 && pop > 7) {
                return 0;
            }
            if rev < i32::MIN / 10 || (rev == i32::MIN / 10 && pop < -8) {
                return 0;
            }
            rev = rev * 10 + pop
        }
        rev
    }

    #[test]
    fn lc_9_tests() {
        struct TestValue {
            input: i32,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input: 123,
                expected: 321,
            },
            TestValue {
                input: -123,
                expected: -321,
            },
            TestValue {
                input: 120,
                expected: 21,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(reverse(t.input), t.expected);
        }
    }
}
