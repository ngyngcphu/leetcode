//! Solution for LeetCode problem #1143: Longest Common Subsequence
//! https://leetcode.com/problems/longest-common-subsequence/

pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let text1_len = text1.len();
        let text2_len = text2.len();

        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();

        let mut dp = vec![vec![0; text2_len + 1]; text1_len + 1];

        for i in (0..text1_len).rev() {
            for j in (0..text2_len).rev() {
                dp[i][j] = if text1[i] == text2[j] {
                    1 + dp[i + 1][j + 1]
                } else {
                    dp[i + 1][j].max(dp[i][j + 1])
                }
            }
        }
        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "def".to_string()),
            0
        );
    }
}
