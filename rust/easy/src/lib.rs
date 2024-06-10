#[cfg(test)]
mod lc_2347 {
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
mod lc_2660 {

    use std::cmp::Ordering::{Equal, Greater, Less};

    fn determine_the_winner_of_a_bowling_game(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        match score(player1).cmp(&score(player2)) {
            Greater => 1,
            Less => 2,
            Equal => 0,
        }
    }

    fn score(player: Vec<i32>) -> i32 {
        player
            .into_iter()
            .fold((0, 0, 0), |(n2, n1, score), x| {
                if n2 == 0 || n1 == 0 {
                    (n1, x, score + 2 * x)
                } else {
                    (n1, x, score + x)
                }
            })
            .2
    }

    #[test]
    fn test_lc_2660_one() {
        assert_eq!(
            determine_the_winner_of_a_bowling_game(vec![4, 10, 7, 9], vec![6, 5, 2, 3]),
            1
        );
    }

    #[test]
    fn test_lc_2660_two() {
        assert_eq!(
            determine_the_winner_of_a_bowling_game(vec![3, 5, 7, 6], vec![8, 10, 10, 2]),
            2
        );
    }

    #[test]
    fn test_lc_2660_three() {
        assert_eq!(
            determine_the_winner_of_a_bowling_game(vec![2, 3], vec![4, 1]),
            0
        );
    }
}

#[cfg(test)]
mod lc_2828 {
    fn is_acronym(words: Vec<String>, s: String) -> bool {
        words
            .into_iter()
            .flat_map(|string| string.chars().next())
            .eq(s.chars())
    }

    #[test]
    fn test_lc_2828_one() {
        assert_eq!(
            is_acronym(
                vec!["alice", "bob", "charlie"]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect(),
                "abc".to_string()
            ),
            true
        );
    }

    #[test]
    fn test_lc_2828_two() {
        assert_eq!(
            is_acronym(
                vec!["an", "apple"].iter().map(|&s| s.to_string()).collect(),
                "a".to_string()
            ),
            false
        );
    }

    #[test]
    fn test_lc_2828_three() {
        assert_eq!(
            is_acronym(
                vec!["never", "gonna", "give", "up", "on", "you"]
                    .iter()
                    .map(|&s| s.to_string())
                    .collect(),
                "ngguoy".to_string()
            ),
            true
        );
    }
}

#[cfg(test)]
mod lc_58 {

    fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().next_back().unwrap().len() as i32
    }

    #[test]
    fn test_lc_58_one() {
        assert_eq!(length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn test_lc_58_two() {
        assert_eq!(
            length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

    #[test]
    fn test_lc_58_three() {
        assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
    }
}

#[cfg(test)]
mod lc_136 {

    fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |pre, curr| pre ^ curr)
    }

    #[test]
    fn test_lc_136_one() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn test_lc_136_two() {
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn test_lc_136_three() {
        assert_eq!(single_number(vec![1]), 1);
    }
}

#[cfg(test)]
mod lc_338 {

    fn count_bits(n: i32) -> Vec<i32> {
        let mut dp = vec![0; (n + 1) as usize];
        let mut offset = 1;
        for i in 1..n + 1 {
            if offset * 2 == i {
                offset = i;
            }
            dp[i as usize] = 1 + dp[(i - offset) as usize]
        }
        dp
    }

    #[test]
    fn test_lc_338() {
        struct TestValues {
            input: i32,
            expected: Vec<i32>,
        }

        let test_cases = [
            TestValues {
                input: 2,
                expected: vec![0, 1, 1],
            },
            TestValues {
                input: 5,
                expected: vec![0, 1, 1, 2, 1, 2],
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(count_bits(t.input), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_118 {

    fn generate(n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![1]];
        for _ in 0..n - 1 {
            let (mut temp, mut row) = (Vec::new(), Vec::new());
            temp.push(0);
            temp.extend(res.last().unwrap().iter());
            temp.push(0);
            for j in 0..=res.last().unwrap().len() {
                row.push(temp[j] + temp[j + 1]);
            }
            res.push(row);
        }
        res
    }

    #[test]
    fn test_lc_118() {
        struct TestValues {
            input: i32,
            expected: Vec<Vec<i32>>,
        }

        let test_cases = [
            TestValues {
                input: 5,
                expected: vec![
                    vec![1],
                    vec![1, 1],
                    vec![1, 2, 1],
                    vec![1, 3, 3, 1],
                    vec![1, 4, 6, 4, 1],
                ],
            },
            TestValues {
                input: 1,
                expected: vec![vec![1]],
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(generate(t.input), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_509 {
    fn fib(n: i32, a: i32, b: i32) -> i32 {
        match n {
            1 => b,
            0 => n,
            _ => fib(n - 1, b, a + b),
        }
    }

    #[test]
    fn lc_509_tests() {
        struct TestValue {
            input1: i32,
            input2: i32,
            input3: i32,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input1: 2,
                input2: 0,
                input3: 1,
                expected: 1,
            },
            TestValue {
                input1: 4,
                input2: 0,
                input3: 1,
                expected: 3,
            },
            TestValue {
                input1: 3,
                input2: 0,
                input3: 1,
                expected: 2,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(fib(t.input1, t.input2, t.input3), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_1137 {
    fn tribonacci(n: i32) -> i32 {
        (0..n).fold((0, 1, 1), |(a, b, c), _| (b, c, a + b + c)).0
    }

    #[test]
    fn lc_1137_tests() {
        struct TestValue {
            input: i32,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input: 4,
                expected: 4,
            },
            TestValue {
                input: 25,
                expected: 1389537,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(tribonacci(t.input), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_70 {
    fn climb_stairs(n: i32) -> i32 {
        (1..n).fold((1, 1), |(a, b), _| (b, a + b)).1
    }

    #[test]
    fn lc_70_tests() {
        struct TestValue {
            input: i32,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input: 2,
                expected: 2,
            },
            TestValue {
                input: 3,
                expected: 3,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(climb_stairs(t.input), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_746 {
    fn min_cost_climbing_stairs(mut cost: Vec<i32>) -> i32 {
        cost.push(0);
        for i in (0..cost.len() - 3).rev() {
            cost[i] += std::cmp::min(cost[i + 1], cost[i + 2]);
        }
        std::cmp::min(cost[0], cost[1])
    }

    #[test]
    fn lc_746_tests() {
        struct TestValue {
            input: Vec<i32>,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input: vec![10, 15, 20],
                expected: 15,
            },
            TestValue {
                input: vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1],
                expected: 6,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(min_cost_climbing_stairs(t.input), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_121 {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut l, mut r, mut max) = (0, 1, 0);
        while r < prices.len() {
            if prices[l] < prices[r] {
                max = std::cmp::max(max, prices[r] - prices[l])
            } else {
                l = r;
            }
            r += 1;
        }
        max
    }

    #[test]
    fn lc_121_tests() {
        struct TestValue {
            input: Vec<i32>,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input: vec![7, 1, 5, 3, 6, 4],
                expected: 5,
            },
            TestValue {
                input: vec![7, 6, 4, 3, 1],
                expected: 0,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(max_profit(t.input), t.expected);
        }
    }
}
