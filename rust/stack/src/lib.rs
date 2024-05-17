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
