//! Solution for LeetCode problem #152: Maximum Product Subarray
//! https://leetcode.com/problems/maximum-product-subarray/description/

pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        nums.iter()
            .skip(1)
            .fold(
                (nums[0], nums[0], nums[0]),
                |(res, cur_min, cur_max), &num| {
                    let new_min = num * cur_min;
                    let new_max = num * cur_max;
                    let new_cur_min = num.min(new_max).min(new_min);
                    let new_cur_max = num.max(new_max).max(new_min);
                    (res.max(new_cur_max), new_cur_min, new_cur_max)
                },
            )
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
        assert_eq!(Solution::max_product(vec![-2, 3, -4]), 24);
    }
}
