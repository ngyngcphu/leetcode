//! Solution for LeetCode problem #70: Climbing Stairs
//! https://leetcode.com/problems/climbing-stairs/

pub struct Solution;

impl Solution {
    pub fn climb_stairs_imperative(n: i32) -> i32 {
        let mut res = 1;
        let mut prev = 1;
        for _ in 0..(n - 1) {
            let temp = res;
            res += prev;
            prev = temp;
        }
        res
    }

    pub fn climb_stairs_functional(n: i32) -> i32 {
        (0..(n - 1))
            .fold((1, 1), |(res, prev), _| (res + prev, res))
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::climb_stairs_imperative(2), 2);
        assert_eq!(Solution::climb_stairs_imperative(3), 3);
    }

    #[test]
    fn test_functional() {
        assert_eq!(Solution::climb_stairs_functional(2), 2);
        assert_eq!(Solution::climb_stairs_functional(3), 3);
    }
}
