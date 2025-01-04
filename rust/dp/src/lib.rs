#[cfg(test)]
mod lc_347 {
    fn best_poker_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        match suits
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            == 1
        {
            true => "Flush".to_string(),
            false => {
                let mut counter = [0; 13];
                ranks.into_iter().for_each(|r| counter[r as usize - 1] += 1);
                match counter.into_iter().max().unwrap() {
                    1 => "High Card".to_string(),
                    2 => "Pair".to_string(),
                    _ => "Three of a Kind".to_string(),
                }
            }
        }
    }

    #[test]
    fn test_lc_2347_one() {
        assert_eq!(
            best_poker_hand(vec![13, 2, 3, 1, 9], vec!['a', 'a', 'a', 'a', 'a']),
            "Flush".to_string()
        );
    }

    #[test]
    fn test_lc_2347_two() {
        assert_eq!(
            best_poker_hand(vec![4, 4, 2, 4, 4], vec!['d', 'a', 'a', 'b', 'c']),
            "Three of a Kind".to_string()
        );
    }

    #[test]
    fn test_lc_2347_three() {
        assert_eq!(
            best_poker_hand(vec![10, 10, 2, 12, 9], vec!['a', 'b', 'c', 'a', 'd']),
            "Pair".to_string()
        );
    }
}

#[cfg(test)]
mod lc_1547 {
    pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        fn dfs(l: i32, r: i32, cuts: &Vec<i32>, dp: &mut HashMap<(i32, i32), i32>) -> i32 {
            // Base case: if the segment is of length 1
            if r - l == 1 {
                return 0;
            }

            // Check if we've already computed this subproblem
            if let Some(&result) = dp.get(&(l, r)) {
                return result;
            }

            // Initialize result to maximum possible value
            let mut res = i32::MAX;

            // Try each possible cut position
            for &c in cuts.iter() {
                if l < c && c < r {
                    let cost = (r - l) + dfs(l, c, cuts, dp) + dfs(c, r, cuts, dp);
                    res = res.min(cost);
                }
            }
            res = if res == i32::MAX { 0 } else { res };
            // Store and return result
            dp.insert((l, r), res);
            res
        }

        // Start the recursion
        dfs(0, n, &cuts, &mut HashMap::new())
    }
}
