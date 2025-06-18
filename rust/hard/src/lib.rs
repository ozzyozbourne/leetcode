#[cfg(test)]

mod scramble_strings {
    use std::collections::HashMap;

    fn is_scramble_top_down(s1: &str, s2: &str) -> bool {
        let mut dp = HashMap::<(&str, &str), bool>::new();
        fn helper<'a>(
            dp: &mut HashMap<(&'a str, &'a str), bool>,
            s1: &'a str,
            s2: &'a str,
        ) -> bool {
            if let Some(&res) = dp.get(&(s1, s2)) {
                return res;
            }
            if s1 == s2 {
                dp.insert((s1, s2), true);
                return true;
            }
            let n = s1.len();
            for split_pos in 1..n {
                let (s1_left, s1_right) = (&s1[..split_pos], &s1[split_pos..]);
                let (s2_left, s2_right) = (&s2[..split_pos], &s2[split_pos..]);

                if helper(dp, s1_left, s2_left) && helper(dp, s1_right, s2_right) {
                    dp.insert((s1, s2), true);
                    return true;
                }

                let s2_left_swap = &s2[..n - split_pos];
                let s2_right_swap = &s2[n - split_pos..];

                if helper(dp, s1_left, s2_right_swap) && helper(dp, s1_right, s2_left_swap) {
                    dp.insert((s1, s2), true);
                    return true;
                }
            }
            dp.insert((s1, s2), false);
            false
        }
        helper(&mut dp, s1, s2)
    }

    fn is_scramble_bottom_up(s1: &str, s2: &str) -> bool {
        let n = s1.len();
        let mut dp = vec![vec![vec![false; n]; n]; n + 1];
        for i in 0..n {
            for j in 0..n {
                dp[1][i][j] = s1[i..i + 1] == s2[j..j + 1];
            }
        }

        for len in 2..n + 1 {
            for i in 0..n + 1 - len {
                for j in 0..n + 1 - len {
                    for new_len in 1..len {
                        dp[len][i][j] |=
                            dp[new_len][i][j] && dp[len - new_len][i + new_len][j + new_len];
                        dp[len][i][j] |=
                            dp[new_len][i][j + len - new_len] && dp[len - new_len][i + new_len][j];
                    }
                }
            }
        }
        dp[n][0][0]
    }

    #[test]
    fn lc_87() {
        for (s1, s2, expected) in vec![
            ("great", "rgeat", true),
            ("abcde", "caebd", false),
            ("a", "a", true),
        ] {
            assert_eq!(is_scramble_top_down(s1, s2), expected);
            assert_eq!(is_scramble_bottom_up(s1, s2), expected);
        }
    }
}
