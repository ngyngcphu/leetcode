//! Solution for LeetCode problem #91: Decode Ways
//! https://leetcode.com/problems/decode-ways/

pub struct Solution;

impl Solution {
    pub fn num_decodings_imperative(s: String) -> i32 {
        let s = s.as_bytes();
        let length = s.len();
        let mut next_val = 1;
        let mut next_next_val = 0;

        for i in (0..length).rev() {
            let mut current_val = 0;
            let current_byte = s[i];
            if current_byte != b'0' {
                current_val = next_val;
                if i + 1 < length {
                    let next_byte = s[i + 1];
                    let two_digit = (current_byte - b'0') * 10 + (next_byte - b'0');
                    if two_digit >= 10 && two_digit <= 26 {
                        current_val += next_next_val;
                    }
                }
            }
            next_next_val = next_val;
            next_val = current_val;
        }
        next_val
    }

    pub fn num_decodings_functional(s: String) -> i32 {
        let s = s.as_bytes();
        let length = s.len();
        (0..length)
            .rev()
            .fold((0, 1), |(next_next_val, next_val), cur| {
                let mut current_val = 0;
                let current_byte = s[cur];
                if current_byte != b'0' {
                    current_val = next_val;
                    if cur + 1 < length {
                        let next_byte = s[cur + 1];
                        let two_digit = (current_byte - b'0') * 10 + (next_byte - b'0');
                        if two_digit >= 10 && two_digit <= 26 {
                            current_val += next_next_val;
                        }
                    }
                }
                (next_val, current_val)
            })
            .1
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imperative() {
        assert_eq!(Solution::num_decodings_imperative("12".to_string()), 2);
        assert_eq!(Solution::num_decodings_imperative("226".to_string()), 3);
        assert_eq!(Solution::num_decodings_imperative("0".to_string()), 0);
    }

    #[test]
    fn test_functional() {
        assert_eq!(Solution::num_decodings_functional("12".to_string()), 2);
        assert_eq!(Solution::num_decodings_functional("226".to_string()), 3);
        assert_eq!(Solution::num_decodings_functional("0".to_string()), 0);
    }
}
