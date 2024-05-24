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
