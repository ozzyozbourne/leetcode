#[cfg(test)]
mod lc_1614_maximum_nesting_depth_of_the_parentheses {

    fn max_depth(s: String) -> i32 {
        let (mut res, mut cur) = (0, 0);
        for c in s.chars() {
            if c == '(' {
                cur += 1
            } else if c == ')' {
                cur -= 1
            }
            res = std::cmp::max(res, cur);
        }
        res
    }

    #[test]
    fn test_lc_1614_one() {
        assert_eq!(max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
    }

    #[test]
    fn test_lc_1614_two() {
        assert_eq!(max_depth("(1)+((2))+(((3)))".to_string()), 3);
    }

    #[test]
    fn test_lc_1614_three() {
        assert_eq!(max_depth("()(())((()()))".to_string()), 3);
    }
}

#[cfg(test)]
mod lc_1598_crawler_log_folder {

    fn min_operations(logs: Vec<&str>) -> i32 {
        let mut step = 0;
        for log in logs.into_iter() {
            match log {
                "../" => step -= (step > 0) as i32,
                "./" => {}
                _ => step += 1,
            }
        }
        step
    }

    #[test]
    fn test_lc_1614_one() {
        assert_eq!(min_operations(vec!["d1/", "../", "../", "../"]), 0);
    }

    #[test]
    fn test_lc_1614_two() {
        assert_eq!(
            min_operations(vec!["d1/", "d2/", "./", "d3/", "../", "d31/"]),
            3
        );
    }

    #[test]
    fn test_lc_1614_three() {
        assert_eq!(min_operations(vec!["d1/", "d2/", "../", "d21/", "./"]), 2);
    }
}

#[cfg(test)]
mod lc_848_shifting_letters {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let (mut ans, mut x) = (
            Vec::with_capacity(s.len()),
            shifts.iter().map(|&n| n as i64).sum::<i64>() % 26,
        );
        for (i, c) in s.bytes().into_iter().enumerate() {
            let index = (c - b'a') as i64;
            ans.push(b'a' + (index + x).rem_euclid(26) as u8);
            x = (x - shifts[i] as i64).rem_euclid(26);
        }
        String::from_utf8(ans).unwrap()
    }

    #[test]
    fn checker() {
        let res = shifting_letters("xrdofd".to_string(), vec![70, 41, 71, 72, 73, 84]);
        println!("{:?}\n", res);
    }
}

#[cfg(test)]
mod lc_1400_contruction_k_palindrome_strings {
    pub fn can_construct(s: String, k: i32) -> bool {
        match s.len() {
            len if len < k as usize => false,
            len if len == k as usize => true,
            _ => {
                s.bytes()
                    .fold(0_i32, |acc, c| acc ^ (1 << (c - b'a')))
                    .count_ones()
                    <= k as u32
            }
        }
    }
}

#[cfg(test)]
mod lc_816_word_subsets {
    pub fn word_subsets(w1: Vec<String>, w2: Vec<String>) -> Vec<String> {
        fn count(w: &String) -> Vec<i32> {
            let mut csum = vec![0; 26];
            for c in w.bytes() {
                csum[(c - b'a') as usize] += 1;
            }
            csum
        }

        let mut bsum = vec![0; 26];

        for w in w2 {
            let csum = count(&w);
            for i in 0..26 {
                bsum[i] = bsum[i].max(csum[i]);
            }
        }

        w1.into_iter()
            .filter(|w| {
                let freq = count(w);
                (0..26).all(|i| freq[i] >= bsum[i])
            })
            .collect()
    }
}
