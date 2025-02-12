//! solution for leetcode problem #33: Search in Rotated Sorted Array
//! https://leetcode.com/problems/search-in-rotated-sorted-array/

pub struct Solution;

impl Solution {
    pub fn search_loop(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if target == nums[mid] {
                return mid as i32;
            }

            if nums[mid] >= nums[left] {
                if target > nums[mid] || target < nums[left] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            } else {
                if target < nums[mid] || target > nums[right] {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
        }

        -1
    }

    pub fn search_recursive(nums: Vec<i32>, target: i32) -> i32 {
        Self::helper(&nums, 0, nums.len() - 1, target)
    }

    fn helper(nums: &[i32], left: usize, right: usize, target: i32) -> i32 {
        if left > right {
            return -1;
        }

        let mid = left + (right - left) / 2;
        if target == nums[mid] {
            return mid as i32;
        }

        if nums[mid] >= nums[left] {
            if target > nums[mid] || target < nums[left] {
                Self::helper(nums, mid + 1, right, target)
            } else {
                Self::helper(nums, left, mid - 1, target)
            }
        } else {
            if target < nums[mid] || target > nums[right] {
                Self::helper(nums, left, mid - 1, target)
            } else {
                Self::helper(nums, mid + 1, right, target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::search_loop(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search_loop(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search_loop(vec![1], 0), -1);
    }

    #[test]
    fn test_recursive() {
        assert_eq!(Solution::search_recursive(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search_recursive(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search_recursive(vec![1], 0), -1);
    }
}
