#[cfg(test)]
pub mod lc_2347 {
    pub fn best_poker_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
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
pub mod lc_2660 {

    use std::cmp::Ordering::{Equal, Greater, Less};

    pub fn determine_the_winner_of_a_bowling_game(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
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
pub mod lc_2828 {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
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
pub mod lc_58 {

    pub fn length_of_last_word(s: String) -> i32 {
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
