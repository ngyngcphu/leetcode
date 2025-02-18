//! Solution for LeetCode problem #139: Word Break
//! https://leetcode.com/problems/word-break/

pub struct Solution;

impl Solution {
    pub fn word_break_imperative(s: String, word_dict: Vec<String>) -> bool {
        let length = s.len();
        let s = s.as_bytes();
        let mut dp = vec![false; length + 1];
        dp[length] = true;
        for i in (0..length).rev() {
            for word in &word_dict {
                let word_len = word.len();
                if i + word_len <= length && s[i..i + word_len] == *word.as_bytes() {
                    dp[i] = dp[i + word_len];
                    if dp[i] {
                        break;
                    }
                }
            }
        }
        dp[0]
    }

    pub fn word_break_functional(s: String, word_dict: Vec<String>) -> bool {
        let length = s.len();
        let s = s.as_bytes();
        let mut dp = vec![false; length + 1];
        dp[length] = true;

        (0..length).rev().for_each(|i| {
            dp[i] = word_dict.iter().any(|word| {
                let word_len = word.len();
                i + word_len <= length && s[i..i + word_len] == *word.as_bytes() && dp[i + word_len]
            });
        });
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::word_break_imperative(
                "leetcode".to_string(),
                vec!["leet".to_string(), "code".to_string()]
            ),
            true
        );
        assert_eq!(
            Solution::word_break_imperative(
                "applepenapple".to_string(),
                vec!["apple".to_string(), "pen".to_string()]
            ),
            true
        );
        assert_eq!(
            Solution::word_break_imperative(
                "catsandog".to_string(),
                vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string()
                ]
            ),
            false
        );
    }

    #[test]
    fn test_functional() {
        assert_eq!(
            Solution::word_break_functional(
                "leetcode".to_string(),
                vec!["leet".to_string(), "code".to_string()]
            ),
            true
        );
        assert_eq!(
            Solution::word_break_functional(
                "applepenapple".to_string(),
                vec!["apple".to_string(), "pen".to_string()]
            ),
            true
        );
        assert_eq!(
            Solution::word_break_functional(
                "catsandog".to_string(),
                vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string()
                ]
            ),
            false
        );
    }
}
