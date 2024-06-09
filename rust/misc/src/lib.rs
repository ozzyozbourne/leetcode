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
