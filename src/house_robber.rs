//! Solution for LeetCode problem #198: House Robber
//! https://leetcode.com/problems/house-robber/

pub struct Solution;

impl Solution {
    pub fn rob_imperative(nums: Vec<i32>) -> i32 {
        let (mut prev_prev, mut prev) = (0, 0);
        for &num in &nums {
            (prev_prev, prev) = (prev, prev.max(num + prev_prev));
        }
        prev
    }

    pub fn rob_functional(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((0, 0), |(prev_prev, prev), num| {
                (prev, prev.max(num + prev_prev))
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::rob_imperative(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob_imperative(vec![2, 7, 9, 3, 1]), 12);
    }

    #[test]
    fn test_functional() {
        assert_eq!(Solution::rob_functional(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob_functional(vec![2, 7, 9, 3, 1]), 12);
    }
}
