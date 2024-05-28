#[cfg(test)]
mod ngr {

    use std::collections::VecDeque;

    fn ngr(nums: Vec<i32>) -> Vec<i32> {
        let (mut stack, mut res): (VecDeque<usize>, Vec<i32>) =
            (VecDeque::new(), vec![-1; nums.len()]);
        for (index, &value) in nums.iter().enumerate() {
            while !stack.is_empty() && nums[*stack.back().unwrap()] < value {
                res[stack.pop_back().unwrap()] = value;
            }
            stack.push_back(index);
        }
        res
    }

    #[test]
    fn test_ngr_one() {
        assert_eq!(ngr(vec![1, 3, 2, 4]), vec![3, 4, 4, -1]);
    }

    #[test]
    fn test_ngr_two() {
        assert_eq!(ngr(vec![5, 4, 3, 2, 1]), vec![-1, -1, -1, -1, -1]);
    }

    #[test]
    fn test_ngr_three() {
        assert_eq!(ngr(vec![20, 70, 30, 80, 60]), vec![70, 80, 80, -1, -1]);
    }
}

#[cfg(test)]
mod nsr {

    use std::collections::VecDeque;

    fn nsr(nums: Vec<i32>) -> Vec<i32> {
        let (mut stack, mut res): (VecDeque<usize>, Vec<i32>) =
            (VecDeque::new(), vec![-1; nums.len()]);
        for (index, &value) in nums.iter().enumerate() {
            while !stack.is_empty() && nums[*stack.back().unwrap()] > value {
                res[stack.pop_back().unwrap()] = value;
            }
            stack.push_back(index);
        }
        res
    }

    #[test]
    fn test_nsr_one() {
        assert_eq!(nsr(vec![1, 3, 2, 4]), vec![-1, 2, -1, -1]);
    }

    #[test]
    fn test_nsr_two() {
        assert_eq!(nsr(vec![5, 4, 3, 2, 1]), vec![4, 3, 2, 1, -1]);
    }

    #[test]
    fn test_nsr_three() {
        assert_eq!(nsr(vec![20, 70, 30, 80, 60]), vec![-1, 30, -1, 60, -1]);
    }
}

#[cfg(test)]
mod ngl {

    use std::collections::VecDeque;

    fn ngl(nums: Vec<i32>) -> Vec<i32> {
        let (mut stack, mut res): (VecDeque<usize>, Vec<i32>) =
            (VecDeque::new(), vec![-1; nums.len()]);
        for (index, &value) in nums.iter().enumerate().rev() {
            while !stack.is_empty() && nums[*stack.back().unwrap()] < value {
                res[stack.pop_back().unwrap()] = value;
            }
            stack.push_back(index);
        }
        res
    }

    #[test]
    fn test_ngl_one() {
        assert_eq!(ngl(vec![1, 3, 2, 4]), vec![-1, -1, 3, -1]);
    }

    #[test]
    fn test_ngl_two() {
        assert_eq!(ngl(vec![5, 4, 3, 2, 1]), vec![-1, 5, 4, 3, 2]);
    }

    #[test]
    fn test_ngl_three() {
        assert_eq!(ngl(vec![20, 70, 30, 80, 60]), vec![-1, -1, 70, -1, 80]);
    }
}

#[cfg(test)]
mod nsl {

    use std::collections::VecDeque;

    fn nsl(nums: Vec<i32>) -> Vec<i32> {
        let (mut stack, mut res): (VecDeque<usize>, Vec<i32>) =
            (VecDeque::new(), vec![-1; nums.len()]);
        for (index, &value) in nums.iter().enumerate().rev() {
            while !stack.is_empty() && nums[*stack.back().unwrap()] > value {
                res[stack.pop_back().unwrap()] = value;
            }
            stack.push_back(index);
        }
        res
    }

    #[test]
    fn test_nsl_one() {
        assert_eq!(nsl(vec![1, 3, 2, 4]), vec![-1, 1, 1, 2]);
    }

    #[test]
    fn test_nsl_two() {
        assert_eq!(nsl(vec![5, 4, 3, 2, 1]), vec![-1, -1, -1, -1, -1]);
    }

    #[test]
    fn test_nsl_three() {
        assert_eq!(nsl(vec![20, 70, 30, 80, 60]), vec![-1, 20, 20, 30, 30]);
    }
}

#[cfg(test)]
mod lc_20_valid_parentheses {

    use std::collections::{HashMap, VecDeque};

    fn is_valid(s: String) -> bool {
        let (mut stack, map): (VecDeque<char>, HashMap<char, char>) = (
            VecDeque::new(),
            [('}', '{'), (')', '('), (']', '[')]
                .iter()
                .cloned()
                .collect(),
        );
        for c in s.chars() {
            if map.contains_key(&c) {
                if !stack.is_empty() && *stack.back().unwrap() == *map.get(&c).unwrap() {
                    _ = stack.pop_back();
                } else {
                    return false;
                }
            } else {
                stack.push_back(c);
            }
        }
        stack.is_empty()
    }

    #[test]
    fn test_lc_20_one() {
        assert_eq!(is_valid("()".to_string()), true);
    }

    #[test]
    fn test_lc_20_two() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn test_lc_20_three() {
        assert_eq!(is_valid("(]".to_string()), false);
    }
}

#[cfg(test)]
mod lc_71_simply_path {

    fn simply_path(path: String) -> String {
        let (mut stack, mut curr) = (Vec::new(), String::new());
        for c in path.chars().chain(std::iter::once('/')) {
            if c == '/' {
                if curr == "..".to_string() {
                    _ = stack.pop();
                } else if !curr.is_empty() && curr != ".".to_string() {
                    stack.push(curr.clone());
                }
                curr.clear();
            } else {
                curr.push(c);
            }
        }
        format!("/{}", stack.join("/"))
    }

    #[test]
    fn test_lc_71_one() {
        assert_eq!(simply_path("/home/".to_string()), "/home".to_string());
    }

    #[test]
    fn test_lc_71_two() {
        assert_eq!(
            simply_path("/home//foo/".to_string()),
            "/home/foo".to_string()
        );
    }

    #[test]
    fn test_lc_71_three() {
        assert_eq!(
            simply_path("/home/user/Documents/../Pictures".to_string()),
            "/home/user/Pictures".to_string()
        );
    }

    #[test]
    fn test_lc_71_four() {
        assert_eq!(
            simply_path("/.../a/../b/c/../d/./".to_string()),
            "/.../b/d".to_string()
        );
    }

    #[test]
    fn test_lc_71_five() {
        assert_eq!(simply_path("/../".to_string()), "/".to_string());
    }
}

#[cfg(test)]
mod lc_496_next_greater_element_i {

    use std::collections::HashMap;

    fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let (mut stack, mut res, mut map) = (
            Vec::<i32>::new(),
            vec![-1; nums1.len()],
            HashMap::<i32, usize>::new(),
        );
        for (index, value) in nums1.into_iter().enumerate() {
            map.insert(value, index);
        }
        for value in nums2.into_iter() {
            while !stack.is_empty() && *stack.last().unwrap() < value {
                res[*map.get(&stack.pop().unwrap()).unwrap()] = value;
            }
            if map.contains_key(&value) {
                stack.push(value);
            }
        }
        res
    }

    #[test]
    fn test_lc_496_one() {
        assert_eq!(
            next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
    }

    #[test]
    fn test_lc_496_two() {
        assert_eq!(
            next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
}

#[cfg(test)]
mod lc_84_largest_rectangle_in_histogram {

    fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let (mut max_area, mut stack) = (0, Vec::<(usize, i32)>::new());
        for (i, &h) in heights.iter().enumerate() {
            let mut start = i;
            while !stack.is_empty() && stack.last().unwrap().1 > h {
                let (index, height) = stack.pop().unwrap();
                max_area = std::cmp::max(max_area, height * (i.checked_sub(index).unwrap()) as i32);
                start = index;
            }
            stack.push((start, h));
        }
        for (i, h) in stack.into_iter() {
            max_area = std::cmp::max(max_area, h * (heights.len().checked_sub(i).unwrap()) as i32);
        }
        max_area
    }

    #[test]
    fn test_lc_84_one() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn test_lc_84_two() {
        assert_eq!(largest_rectangle_area(vec![2, 4]), 4);
    }
}

#[cfg(test)]
mod lc_32_longest_valid_parentheses {

    fn longest_valid_parentheses(s: String) -> i32 {
        let (mut stack, mut res) = (vec![-1], 0);
        for (i, c) in s.char_indices() {
            if c == '(' {
                stack.push(i as i32);
            } else {
                _ = stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32)
                } else {
                    res = std::cmp::max(res, i as i32 - *stack.last().unwrap());
                }
            }
        }
        res
    }

    #[test]
    fn test_lc_32_one() {
        assert_eq!(longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn test_lc_32_two() {
        assert_eq!(longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn test_lc_32_three() {
        assert_eq!(longest_valid_parentheses("".to_string()), 0);
    }
}

#[cfg(test)]
mod lc_2696_minimum_string_length_after_removing_substrings {

    fn min_length(s: String) -> i32 {
        let (mut stack, set) = (Vec::new(), vec!["AB", "CD"]);
        for c in s.chars() {
            if !stack.is_empty()
                && set.contains(&format!("{}{}", *stack.last().unwrap(), c).as_str())
            {
                _ = stack.pop();
            } else {
                stack.push(c)
            }
        }
        stack.len() as i32
    }

    #[test]
    fn test_lc_2696_one() {
        assert_eq!(min_length("ABFCACDB".to_string()), 2);
    }

    #[test]
    fn test_lc_2696_two() {
        assert_eq!(min_length("ACBBD".to_string()), 5);
    }
}

#[cfg(test)]
mod lc_1700_number_of_students_unable_to_eat_lunch {

    fn count_students(student: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let (mut zero_count, mut one_count) = (0, 0);
        for s in student.into_iter() {
            match s {
                0 => zero_count += 1,
                _ => one_count += 1,
            }
        }
        for s in sandwiches.into_iter() {
            match s {
                0 if zero_count == 0 => return one_count,
                1 if one_count == 0 => return zero_count,
                0 => zero_count -= 1,
                _ => one_count -= 1,
            }
        }
        0
    }

    #[test]
    fn test_lc_1700_one() {
        assert_eq!(count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]), 0);
    }

    #[test]
    fn test_lc_1700_two() {
        assert_eq!(
            count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}

#[cfg(test)]
mod lc_1544_make_the_string_great {

    fn make_good(s: String) -> String {
        let mut stack = Vec::<char>::new();
        for c in s.chars() {
            if !stack.is_empty()
                && !stack.last().unwrap().eq(&c)
                && stack.last().unwrap().to_lowercase().next().unwrap()
                    == c.to_lowercase().next().unwrap()
            {
                _ = stack.pop()
            } else {
                stack.push(c);
            }
        }
        stack.iter().collect()
    }

    #[test]
    fn test_lc_1544_one() {
        assert_eq!(make_good("leEeetcode".to_string()), "leetcode".to_string());
    }

    #[test]
    fn test_lc_1544_two() {
        assert_eq!(make_good("abBAcC".to_string()), "".to_string());
    }

    #[test]
    fn test_lc_1544_three() {
        assert_eq!(make_good("s".to_string()), "s".to_string());
    }
}

#[cfg(test)]
mod lc_1021_remove_utermost_parentheses {

    fn remove_outer_parentheses(s: String) -> String {
        let (mut answer, mut level) = (String::new(), 0);
        for c in s.chars() {
            match c {
                '(' => {
                    if level > 0 {
                        answer.push(c);
                    }
                    level += 1;
                }
                _ => {
                    level -= 1;
                    if level > 0 {
                        answer.push(c);
                    }
                }
            }
        }
        answer
    }

    #[test]
    fn test_lc_1021_one() {
        assert_eq!(
            remove_outer_parentheses("(()())(())".to_string()),
            "()()()".to_string()
        );
    }

    #[test]
    fn test_lc_1021_two() {
        assert_eq!(
            remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())".to_string()
        );
    }

    #[test]
    fn test_lc_1021_three() {
        assert_eq!(remove_outer_parentheses("()()".to_string()), "".to_string());
    }
}

#[cfg(test)]
mod lc_1475_final_prices_with_a_special_discount_in_a_shop {

    fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        for i in 0..prices.len() {
            while !stack.is_empty() && prices[*stack.last().unwrap()] >= prices[i] {
                prices[stack.pop().unwrap()] -= prices[i];
            }
            stack.push(i);
        }
        prices
    }

    #[test]
    fn test_lc_1475() {
        struct TestValues {
            input: Vec<i32>,
            expected: Vec<i32>,
        }

        let test_cases = [
            TestValues {
                input: vec![8, 4, 6, 2, 3],
                expected: vec![4, 2, 4, 2, 3],
            },
            TestValues {
                input: vec![1, 2, 3, 4, 5],
                expected: vec![1, 2, 3, 4, 5],
            },
            TestValues {
                input: vec![10, 1, 1, 6],
                expected: vec![9, 0, 1, 6],
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(final_prices(t.input), t.expected);
        }
    }
}
