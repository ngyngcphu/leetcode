//! Solution for LeetCode problem #217: Contains Duplicate
//! https://leetcode.com/problems/contains-duplicate/

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut exists = std::collections::HashSet::new();
        nums.into_iter().any(|num| !exists.insert(num))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
            true
        );
    }
}
