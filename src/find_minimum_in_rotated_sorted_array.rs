//! Solution for LeetCode problem #153: Find Minimum in Rotated Sorted Array
//! https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

pub struct Solution;

impl Solution {
    pub fn find_min_loop(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            if nums[left] < nums[right] {
                return res.min(nums[left]);
            } else {
                let mid = left + (right - left) / 2;
                res = res.min(nums[mid]);
                if nums[mid] >= nums[left] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        res
    }

    pub fn find_min_recursive(nums: Vec<i32>) -> i32 {
        Self::helper(&nums, 0, nums.len() - 1, nums[0])
    }

    fn helper(nums: &[i32], left: usize, right: usize, current_min: i32) -> i32 {
        if left > right {
            current_min
        } else {
            if nums[left] < nums[right] {
                current_min.min(nums[left])
            } else {
                let mid = left + (right - left) / 2;
                let new_min = current_min.min(nums[mid]);
                if nums[mid] >= nums[left] {
                    Self::helper(nums, mid + 1, right, new_min)
                } else {
                    Self::helper(nums, left, mid - 1, new_min)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop() {
        assert_eq!(Solution::find_min_loop(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min_loop(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min_loop(vec![11, 13, 15, 17]), 11);
    }

    #[test]
    fn test_recursive() {
        assert_eq!(Solution::find_min_recursive(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min_recursive(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min_recursive(vec![11, 13, 15, 17]), 11);
    }
}
