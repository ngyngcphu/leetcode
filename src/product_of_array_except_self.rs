//! Solution for LeetCode problem #238: Product of Array Except Self
//! https://leetcode.com/problems/product-of-array-except-self/

pub struct Solution;

impl Solution {
    pub fn product_except_self_imperative(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![1; n];

        let mut prefix = 1;
        for i in 0..n {
            res[i] = prefix;
            prefix *= nums[i];
        }

        let mut postfix = 1;
        for i in (0..n).rev() {
            res[i] *= postfix;
            postfix *= nums[i];
        }

        res
    }

    pub fn product_except_self_functional(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![1; n];

        (0..n).fold(1, |prefix, i| {
            res[i] = prefix;
            prefix * nums[i]
        });

        (0..n).rev().fold(1, |postfix, i| {
            res[i] *= postfix;
            postfix * nums[i]
        });

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self_imperative() {
        assert_eq!(
            Solution::product_except_self_imperative(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self_imperative(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }

    #[test]
    fn test_product_except_self_functional() {
        assert_eq!(
            Solution::product_except_self_functional(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self_functional(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
