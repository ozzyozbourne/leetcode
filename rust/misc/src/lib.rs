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
    fn lc_509_tests() {
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
