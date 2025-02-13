//! Solution for LeetCode problem #191: Number of 1 Bits
//! https://leetcode.com/problems/number-of-1-bits/

pub struct Solution;

impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut n = n;
        let mut count = 0;
        while n > 0 {
            count += 1;
            n &= n - 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::hamming_weight(11), 3);
        assert_eq!(Solution::hamming_weight(128), 1);
        assert_eq!(Solution::hamming_weight(2147483645), 30);
    }
}
