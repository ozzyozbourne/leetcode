#[cfg(test)]
mod lc_42_trapping_rain_water {

    fn trap(height: Vec<i32>) -> i32 {
        let (mut l, mut l_max, mut res, mut r_max, mut r) = (
            0_usize,
            height[0],
            0,
            *height.last().unwrap(),
            height.len() - 1_usize,
        );
        while l < r {
            if l_max < r_max {
                l += 1;
                l_max = std::cmp::max(l_max, height[l]);
                res += l_max - height[l];
            } else {
                r -= 1;
                r_max = std::cmp::max(r_max, height[r]);
                res += r_max - height[r];
            }
        }
        res
    }

    #[test]
    fn test_lc_42_one() {
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn test_lc_42_two() {
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
