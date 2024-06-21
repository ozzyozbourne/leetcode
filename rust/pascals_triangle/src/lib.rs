#[cfg(test)]
mod lc_2099 {

    fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut res = vec![poured as f64];
        for row in 1..=query_row {
            let mut cur_row = vec![0.0; (row + 1) as usize];
            for i in 0..res.len() {
                let extra = res[i] - 1.0;
                if extra > 0.0 {
                    cur_row[i] += 0.5 * extra;
                    cur_row[i + 1] += 0.5 * extra;
                }
            }
            res = cur_row;
        }
        if 1.0 < res[query_glass as usize] {
            1.0
        } else {
            res[query_glass as usize]
        }
    }

    #[test]
    fn lc_509_tests() {
        struct TestValue {
            input1: i32,
            input2: i32,
            input3: i32,
            expected: f64,
        }

        let test_cases = [
            TestValue {
                input1: 1,
                input2: 1,
                input3: 1,
                expected: 0.0,
            },
            TestValue {
                input1: 2,
                input2: 1,
                input3: 1,
                expected: 0.5,
            },
            TestValue {
                input1: 100000009,
                input2: 33,
                input3: 17,
                expected: 1.0,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(champagne_tower(t.input1, t.input2, t.input3), t.expected);
        }
    }
}

