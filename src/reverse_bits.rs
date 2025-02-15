//! Solution for LeetCode problem #190: Reverse Bits
//! https://leetcode.com/problems/reverse-bits/

pub struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        (0..32).fold(0, |acc, i| acc | ((x >> i) & 1) << (31 - i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::reverse_bits(0b00000010100101000001111010011100),
            964176192
        );
        assert_eq!(
            Solution::reverse_bits(0b11111111111111111111111111111101),
            3221225471
        );
    }
}
