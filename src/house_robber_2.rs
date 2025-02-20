//! Solution for LeetCode problem #213: House Robber II
//! https://leetcode.com/problems/house-robber-ii/

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        Self::helper(&nums[..nums.len() - 1])
            .max(Self::helper(&nums[1..]))
            .max(nums[0])
    }

    fn helper(nums: &[i32]) -> i32 {
        nums.iter()
            .fold((0, 0), |(prev_prev, prev), num| {
                (prev, prev.max(prev_prev + num))
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![1, 2, 3]), 3);
    }
}
