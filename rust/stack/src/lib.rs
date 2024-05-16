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
