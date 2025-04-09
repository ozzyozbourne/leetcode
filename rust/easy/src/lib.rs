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
        let (mut dp, mut offset) = (vec![0; (n + 1) as usize], 1);
        for i in 1..=n {
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

#[cfg(test)]
mod lc_392 {
    fn is_subsequence(s: String, t: String) -> bool {
        let (mut l, mut r, s1, t1): (usize, usize, Vec<char>, Vec<char>) =
            (0, 0, s.chars().collect(), t.chars().collect());
        while l < s1.len() && r < t1.len() {
            if s1[l] == t1[r] {
                l += 1;
            }
            r += 1;
        }
        l == s1.len()
    }

    #[test]
    fn lc_392_tests() {
        struct TestValue {
            input1: String,
            input2: String,
            expected: bool,
        }

        let test_cases = [
            TestValue {
                input1: String::from("abc"),
                input2: String::from("ahbgdc"),
                expected: true,
            },
            TestValue {
                input1: String::from("axc"),
                input2: String::from("ahbgdc"),
                expected: false,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(is_subsequence(t.input1, t.input2), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_1688 {
    fn max_repeating(sequence: String, word: String) -> i32 {
        let max_word = word.repeat(sequence.len() / word.len());
        let mut n = max_word.len();
        while sequence.find(&max_word[..n]).is_none() {
            n -= word.len();
        }
        (n / word.len()) as _
    }

    #[test]
    fn lc_392_tests() {
        struct TestValue {
            input1: String,
            input2: String,
            expected: i32,
        }

        let test_cases = [
            TestValue {
                input1: String::from("ababc"),
                input2: String::from("ab"),
                expected: 2,
            },
            TestValue {
                input1: String::from("ababc"),
                input2: String::from("ba"),
                expected: 1,
            },
            TestValue {
                input1: String::from("ababc"),
                input2: String::from("ac"),
                expected: 0,
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(max_repeating(t.input1, t.input2), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_2900 {
    fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let (mut res, mut prev) = (Vec::new(), -1);
        for (c, g) in words.into_iter().zip(groups.into_iter()) {
            if g != prev {
                prev = g;
                res.push(c);
            }
        }
        res
    }

    #[test]
    fn lc_2900_tests() {
        struct TestValue {
            input1: Vec<String>,
            input2: Vec<i32>,
            expected: Vec<String>,
        }

        let test_cases = [
            TestValue {
                input1: vec![String::from("c")],
                input2: vec![0],
                expected: vec![String::from("c")],
            },
            TestValue {
                input1: vec![String::from("d")],
                input2: vec![0],
                expected: vec![String::from("d")],
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(get_longest_subsequence(t.input1, t.input2), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_119 {
    fn get_row(row_index: i32) -> Vec<i32> {
        let mut res = vec![1];
        for _ in 0..row_index {
            let mut next_row = vec![0; res.len() + 1];
            for (j, v) in res.into_iter().enumerate() {
                next_row[j] += v;
                next_row[j + 1] += v;
            }
            res = next_row;
        }
        res
    }

    #[test]
    fn lc_119_tests() {
        struct TestValue {
            input: i32,
            expected: Vec<i32>,
        }

        let test_cases = [
            TestValue {
                input: 3,
                expected: vec![1, 3, 3, 1],
            },
            TestValue {
                input: 0,
                expected: vec![1],
            },
            TestValue {
                input: 1,
                expected: vec![1, 1],
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(get_row(t.input), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_3024 {
    fn triangle_type(nums: Vec<i32>) -> String {
        if nums[0] + nums[1] > nums[2] && nums[0] + nums[2] > nums[1] && nums[1] + nums[2] > nums[0]
        {
            if nums[0] == nums[1] && nums[1] == nums[2] {
                "equilateral".to_string()
            } else if nums[0] == nums[1] || nums[0] == nums[2] || nums[1] == nums[2] {
                "isosceles".to_string()
            } else {
                "scalene".to_string()
            }
        } else {
            "none".to_string()
        }
    }

    #[test]
    fn lc_3024_tests() {
        struct TestValue {
            input: Vec<i32>,
            expected: String,
        }

        let test_cases = [
            TestValue {
                input: vec![3, 3, 3],
                expected: "equilateral".to_string(),
            },
            TestValue {
                input: vec![3, 4, 5],
                expected: "scalene".to_string(),
            },
        ];

        for t in test_cases.into_iter() {
            assert_eq!(triangle_type(t.input), t.expected);
        }
    }
}

#[cfg(test)]
mod lc_2381 {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let (mut diff, mut ch, mut prefix_sum) = (vec![0; s.len() + 1], s.into_bytes(), 0);

        for (st, en, dr) in shifts
            .into_iter()
            .map(|i| (i[0] as usize, i[1] as usize, i[2] as usize))
        {
            diff[st] += if dr == 1 { 1 } else { -1 };
            diff[en + 1] += if dr == 1 { -1 } else { 1 };
        }

        for i in 0..ch.len() {
            prefix_sum += diff[i];
            ch[i] = b'a' + (((ch[i] - b'a') as i32 + prefix_sum) % 26) as u8;
        }

        String::from_utf8(ch).unwrap()
    }
}

#[cfg(test)]
mod lc_1769 {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let (mut ops, mut cnt, mut res, boxes) = (0, 0, vec![0; boxes.len()], boxes.into_bytes());

        for (i, &v) in boxes.iter().enumerate() {
            res[i] += ops;
            cnt += if v == b'1' { 1 } else { 0 };
            ops += cnt;
        }

        let (mut ops, mut cnt) = (0, 0);

        for (i, v) in boxes.into_iter().rev().enumerate() {
            res[i] += ops;
            cnt += if v == b'1' { 1 } else { 0 };
            ops += cnt;
        }

        res
    }
}

#[cfg(test)]
mod lc_208 {
    use std::collections::HashMap;

    #[derive(Default)]
    struct Trie {
        children: HashMap<char, Trie>,
        is_leaf: bool,
    }

    impl Trie {
        fn new() -> Self {
            Trie::default()
        }

        fn insert(&mut self, word: String) {
            word.chars()
                .fold(self, |node, c| node.children.entry(c).or_default())
                .is_leaf = true;
        }

        fn get(&self, word: String) -> Option<&Trie> {
            word.chars().try_fold(self, |node, c| node.children.get(&c))
        }

        fn search(&self, word: String) -> bool {
            self.get(word).map_or(false, |node| node.is_leaf)
        }

        fn search_with(&self, prefix: String) -> bool {
            self.get(prefix).is_some()
        }
    }
}

#[cfg(test)]
mod lc_1162 {
    use std::collections::VecDeque;
    pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let (n, mut q, mut distance, dirs) = (
            grid.len(),
            VecDeque::new(),
            0,
            vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
        );

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    q.push_back((i as i32, j as i32));
                }
            }
        }

        while !q.is_empty() {
            for _ in 0..q.len() {
                let (x, y) = q.pop_front().unwrap();
                for (dx, dy) in dirs.iter() {
                    let (i, j) = (x + dx, y + dy);
                    if (0 <= i && i < n as i32)
                        && (0 <= j && j < n as i32)
                        && grid[i as usize][j as usize] == 0
                    {
                        grid[i as usize][j as usize] = 1;
                        q.push_back((i, j));
                    }
                }
            }
            distance += 1;
        }
        distance - 1
    }
}

#[cfg(test)]
mod lc_93 {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        if s.len() > 12 {
            return vec![];
        }
        let (mut res, s) = (vec![], s.as_bytes());

        fn backtract(res: &mut Vec<String>, i: usize, dots: usize, cur: &mut String, s: &[u8]) {
            if dots > 4 {
                return;
            }
            if dots == 4 && i == s.len() {
                res.push(cur[..cur.len() - 1].to_string());
                return;
            }
            for j in i..(i + 3).min(s.len()) {
                if j > i && s[0] == b'0' {
                    return;
                }
                if std::str::from_utf8(&s[i..=j])
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
                    > 255
                {
                    return;
                }
                let trun = cur.len();
                cur.push_str(std::str::from_utf8(&s[i..=j]).unwrap());
                cur.push('.');
                backtract(res, j + 1, dots + 1, cur, s);
                cur.truncate(trun);
            }
        }

        backtract(&mut res, 0, 0, &mut String::new(), &s);
        res
    }
}
#[cfg(test)]
mod lc_542 {
    use std::collections::VecDeque;

    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (mut res, mut q) = (
            vec![vec![false; mat[0].len()]; mat.len()],
            VecDeque::<(i32, i32)>::new(),
        );
        let (dirs, mut distance) = (vec![(0, 1), (0, -1), (1, 0), (-1, 0)], 1);

        for (i, r) in mat.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 0 {
                    res[i][j] = true;
                    q.push_back((i as i32, j as i32));
                }
            }
        }

        while !q.is_empty() {
            for _ in 0..q.len() {
                let (x, y) = q.pop_front().unwrap();
                for (dx, dy) in dirs.iter() {
                    let (i, j) = (dx + x, dy + y);
                    if (0 <= i && i < mat.len() as i32)
                        && (0 <= j && j < mat[0].len() as i32)
                        && !res[i as usize][j as usize]
                    {
                        q.push_back((i, j));
                        res[i as usize][j as usize] = true;
                        mat[i as usize][j as usize] = distance;
                    }
                }
            }
            distance += 1;
        }
        mat
    }
}

#[cfg(test)]
mod lc_996 {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let (mut visited, mut q, mut fresh, mut time, r, c) = (
            vec![vec![false; grid[0].len()]; grid.len()],
            std::collections::VecDeque::<(i32, i32)>::new(),
            0,
            0,
            grid.len() as i32,
            grid[0].len() as i32,
        );
        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (i, row) in grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == 1 {
                    fresh += 1;
                } else if c == 0 {
                    visited[i][j] = true;
                    q.push_back((i as i32, j as i32));
                }
            }
        }

        while !q.is_empty() {
            for _ in 0..q.len() {
                let (x, y) = q.pop_front().unwrap();
                for (dx, dy) in dirs.iter() {
                    let (i, j) = (dx + x, dy + y);
                    if (0 <= i && i < r)
                        && (0 <= j && j < c)
                        && !visited[i as usize][j as usize]
                        && grid[i as usize][j as usize] == 1
                    {
                        fresh -= 1;
                        visited[i as usize][j as usize] = true;
                        q.push_back((i, j));
                    }
                }
            }
            time += 1;
        }

        if fresh != 0 {
            -1
        } else {
            time
        }
    }
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut counter = vec![0; 26];

    for (sc, tc) in s.bytes().zip(t.bytes()) {
        counter[(sc - b'a') as usize] += 1;
        counter[(tc - b'a') as usize] += 1;
    }

    counter.into_iter().all(|i| i == 0)
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::<i32, i32>::new();
    for (i, v) in nums.into_iter().enumerate().map(|(i, v)| (i as i32, v)) {
        let t = target - v;
        if map.contains_key(&t) {
            return vec![map.remove_entry(&t).unwrap().1, i];
        }
        map.insert(v, i);
    }
    vec![]
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = std::collections::HashMap::<[u8; 26], Vec<String>>::new();

    for s in strs {
        let mut count = [0; 26];
        for c in s.bytes() {
            count[(c - b'a') as usize] += 1;
        }
        map.entry(count).or_default().push(s);
    }
    map.into_values().collect()
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; nums.len()];
    let (mut prefix, mut postfix) = (1, 1);
    for (i, v) in nums.iter().enumerate() {
        res[i] = prefix;
        prefix *= v;
    }
    for (i, v) in (0..nums.len()).rev().zip(nums.into_iter().rev()) {
        res[i] *= postfix;
        postfix *= v;
    }
    res
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let counter = nums.iter().fold(
        std::collections::HashMap::<i32, usize>::new(),
        |mut acc, &num| {
            *acc.entry(num).or_default() += 1;
            acc
        },
    );
    let (mut freq, mut res) = (vec![vec![]; nums.len() + 1], vec![0; k as usize]);

    for (k, v) in counter.into_iter() {
        freq[v].push(k);
    }

    while let Some(num) = freq.pop() {
        for n in num {
            res.push(n);
            if res.len() == k as usize {
                return res;
            }
        }
    }
    res
}

pub fn top_k_frequent_using_min_heap(nums: Vec<i32>, size: i32) -> Vec<i32> {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashMap},
    };
    let count = nums
        .iter()
        .fold(HashMap::<i32, usize>::new(), |mut acc, &n| {
            *acc.entry(n).or_default() += 1;
            acc
        });

    let heap = count.into_iter().fold(
        BinaryHeap::<Reverse<(usize, i32)>>::new(),
        |mut acc, (k, v)| {
            acc.push(Reverse((v, k)));
            if acc.len() > size as usize {
                _ = acc.pop();
            }
            acc
        },
    );

    let mut result = heap.into_iter().fold(
        Vec::with_capacity(size as usize),
        |mut acc, Reverse((_, num))| {
            acc.push(num);
            acc
        },
    );

    result.reverse();
    result
}
pub fn is_palindrome(s: String) -> bool {
    let (s, mut l, mut r) = (s.chars().collect::<Vec<char>>(), 0, s.len() - 1);
    while l < r {
        while l < r && s[l].is_ascii_alphanumeric() {
            l += 1;
        }
        while l < r && s[r].is_ascii_alphanumeric() {
            r -= 1;
        }
        if s[l].to_ascii_lowercase() != s[r].to_ascii_lowercase() {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

pub fn trap(height: Vec<i32>) -> i32 {
    let mut stack: Vec<usize> = Vec::new(); // Store indices, not heights
    let mut res = 0;

    for i in 0..height.len() {
        while !stack.is_empty() && height[i] >= height[*stack.last().unwrap()] {
            let mid_idx = stack.pop().unwrap();
            let mid = height[mid_idx]; // Valley bottom height

            if !stack.is_empty() {
                let right = height[i];
                let left = height[*stack.last().unwrap()];

                // Calculate trapped water height and width
                let h = std::cmp::min(right, left) - mid;
                let w = (i - stack.last().unwrap() - 1) as i32;

                res += h * w;
            }
        }

        stack.push(i);
    }

    res
}

pub fn character_replacement(s: String, k: i32) -> i32 {
    let (mut l, mut res, mut maxf, mut counter, s) = (
        0,
        0,
        0,
        std::collections::HashMap::<char, usize>::new(),
        s.chars().collect::<Vec<char>>(),
    );
    for (r, &v) in s.iter().enumerate() {
        *counter.entry(v).or_default() += 1;
        maxf = maxf.max(*counter.get(&v).unwrap());
        while (r - l + 1 - maxf) as i32 > k {
            *counter.get_mut(&s[l]).unwrap() -= 1;
            l += 1;
        }
        res = res.max((r - l + 1) as i32);
    }
    res
}

pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut result = Vec::new();
    let mut deque = std::collections::VecDeque::new(); // Store indices
    let mut left = 0;

    for right in 0..nums.len() {
        // Remove elements smaller than current from the back
        // (they can't be the maximum in future windows)
        while !deque.is_empty() && deque.back().map_or(false, |&idx| nums[idx] < nums[right]) {
            deque.pop_back();
        }

        deque.push_back(right);

        // Remove elements outside the current window
        if !deque.is_empty() && deque.front().map_or(false, |&idx| left > idx) {
            deque.pop_front();
        }

        // When window reaches size k, add the maximum to the result
        if right + 1 >= k {
            if let Some(&idx) = deque.front() {
                result.push(nums[idx]);
            }
            left += 1;
        }
    }

    result
}
struct Codec {}
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut result = String::new();
        for s in strs {
            result.push_str(&s.len().to_string());
            result.push('#');
            result.push_str(&s);
        }
        result
    }

    fn decode(&self, s: String) -> Vec<String> {
        let (mut result, mut i, s) = (Vec::new(), 0, s.as_bytes());

        while i < s.len() {
            let mut j = i;
            while s[j] != b'#' {
                j += 1;
            }
            let len = std::str::from_utf8(&s[i..j])
                .unwrap()
                .parse::<usize>()
                .unwrap();
            i = j + 1;
            j = i + len;
            let decoded_str = std::str::from_utf8(&s[i..j]).unwrap();
            result.push(decoded_str.to_string());
            i = j;
        }
        result
    }
}

pub fn min_window(s: String, t: String) -> String {
    use std::collections::HashMap;
    let (counter_t, mut counter_s) = (
        t.chars().fold(HashMap::<char, usize>::new(), |mut acc, c| {
            *acc.entry(c).or_default() += 1;
            acc
        }),
        HashMap::<char, usize>::new(),
    );

    let (mut have, need, mut res, mut reslen, mut l, s_arr) = (
        0,
        counter_t.len(),
        (0, 0),
        usize::MAX,
        0,
        s.chars().collect::<Vec<char>>(),
    );

    for (r, c) in s.chars().enumerate() {
        *counter_s.entry(c).or_default() += 1;
        if counter_t.contains_key(&c) && counter_t.get(&c).unwrap() == counter_s.get(&c).unwrap() {
            have += 1;
        }
        while have == need {
            if r - l + 1 < reslen {
                res = (l, r);
                reslen = r - l + 1;
            }
            *counter_s.get_mut(&s_arr[l]).unwrap() -= 1;
            if counter_t.contains_key(&s_arr[l])
                && counter_s.get(&s_arr[l]).unwrap() < counter_t.get(&s_arr[l]).unwrap()
            {
                have -= 1;
            }
            l += 1;
        }
    }

    if reslen == usize::MAX {
        "".to_string()
    } else {
        s[res.0..res.1 + 1].to_string()
    }
}

pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
    let (mut res, mut left, mut right) = (0, 0, nums.len() as i32);
    let mut counter = nums.into_iter().fold(
        std::collections::HashMap::<i32, i32>::new(),
        |mut acc, i| {
            *acc.entry(i).or_default() += 1;
            acc
        },
    );
    for (_, frq) in counter.drain() {
        right -= frq;
        res += left * frq * right;
        left += frq;
    }
    res
}
use std::collections::VecDeque;
pub fn shortest_alternating_paths(
    n: i32,
    red_edges: Vec<Vec<i32>>,
    blue_edges: Vec<Vec<i32>>,
) -> Vec<i32> {
    let mut adj = red_edges
        .into_iter()
        .map(|i| (i[0] as usize, i[1] as usize))
        .fold(vec![Vec::new(); n as usize], |mut acc, (src, dst)| {
            acc[src].push((dst, 0));
            acc
        });

    adj = blue_edges
        .into_iter()
        .map(|i| (i[0] as usize, i[1] as usize))
        .fold(adj, |mut acc, (src, dst)| {
            acc[src].push((dst, 1));
            acc
        });

    let (mut res, mut visited, mut queue) = (
        vec![-1; n as usize],
        vec![[false, false]; n as usize],
        VecDeque::new(),
    );
    (visited[0][0], visited[0][1], res[0]) = (true, true, 0);
    queue.push_back((0, 0, 2));

    while !queue.is_empty() {
        let (node, steps, prev_color) = queue.pop_front().unwrap();
        for &(neighbour, color) in adj.get(node).unwrap() {
            if !visited[neighbour][color] && color != prev_color {
                visited[neighbour][color] = true;
                queue.push_back((neighbour, steps + 1, color));
                if res[neighbour] == -1 {
                    res[neighbour] = steps + 1;
                }
            }
        }
    }

    res
}

pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    use std::collections::{HashSet, VecDeque};
    let (rows, cols) = (grid.len(), grid[0].len());
    let (target, state) = ((rows - 1, cols - 1), (0_usize, 0_usize, k));

    if k as usize >= rows + cols - 2 {
        return (rows + cols - 2) as _;
    }
    let (mut queue, mut seen) = (
        vec![(0, state)]
            .into_iter()
            .collect::<VecDeque<(usize, (usize, usize, i32))>>(),
        vec![state]
            .into_iter()
            .collect::<HashSet<(usize, usize, i32)>>(),
    );

    while !queue.is_empty() {
        let (steps, (row, col, k)) = queue.pop_front().unwrap();
        if row == target.0 && col == target.1 {
            return steps as _;
        }
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nr, nc) = (row as i32 + dx, col as i32 + dy);
            if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                let new_state = (nr as usize, nc as usize, k - grid[nr as usize][nc as usize]);
                if k >= 0 && !seen.contains(&new_state) {
                    seen.insert(new_state);
                    queue.push_back((steps + 1, (nr as usize, nc as usize, k)));
                }
            }
        }
    }
    -1
}

pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
    fn rpc(tokens: &mut Vec<String>) -> i32 {
        let token = tokens.pop().unwrap();
        if !["+", "-", "/", "*"].contains(&token.as_str()) {
            return token.parse().unwrap();
        }
        let (right, left) = (rpc(tokens), rpc(tokens));
        match token.as_str() {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            _ => left / right,
        }
    }
    rpc(&mut tokens)
}
