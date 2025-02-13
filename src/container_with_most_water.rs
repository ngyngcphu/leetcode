//! Solution for LeetCode problem #11: Container With Most Water
//! https://leetcode.com/problems/container-with-most-water/

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            res = res.max((right - left) as i32 * height[left].min(height[right]));

            if height[left] <= height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}
