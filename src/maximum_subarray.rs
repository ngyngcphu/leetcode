//! Solution for LeetCode problem #53: Maximum Subarray
//! https://leetcode.com/problems/maximum-subarray/

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold((nums[0], 0), |(max_sub, cur_sum), num| {
                let cur_sum = cur_sum.max(0) + num;
                (max_sub.max(cur_sum), cur_sum)
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
