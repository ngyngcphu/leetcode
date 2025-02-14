//! Solution for LeetCode problem #268: Missing Number
//! https://leetcode.com/problems/missing-number/

pub struct Solution;

impl Solution {
    pub fn missing_number_xor(nums: Vec<i32>) -> i32 {
        (0..nums.len()).fold(nums.len() as i32, |acc, cur| acc ^ cur as i32 ^ nums[cur])
    }

    pub fn missing_number_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        n * (n + 1) / 2 - nums.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        assert_eq!(Solution::missing_number_xor(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number_xor(vec![0, 1]), 2);
        assert_eq!(
            Solution::missing_number_xor(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
            8
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(Solution::missing_number_sum(vec![3, 0, 1]), 2);
        assert_eq!(Solution::missing_number_sum(vec![0, 1]), 2);
        assert_eq!(
            Solution::missing_number_sum(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
            8
        );
    }
}
