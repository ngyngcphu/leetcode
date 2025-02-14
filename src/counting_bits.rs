//! Solution for LeetCode problem #338: Counting Bits
//! https://leetcode.com/problems/counting-bits/

pub struct Solution;

impl Solution {
    pub fn count_bits_imperative(n: i32) -> Vec<i32> {
        let mut dp = vec![0; n as usize + 1];
        for i in 1..=n as usize {
            dp[i] = dp[i >> 1] + (i & 1) as i32;
        }
        dp
    }

    pub fn count_bits_functional(n: i32) -> Vec<i32> {
        (1..=n as usize).fold(vec![0], |mut dp, i| {
            dp.push(dp[i >> 1] + (i & 1) as i32);
            dp
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::count_bits_imperative(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits_imperative(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
