//! Solution for LeetCode problem #1: Two Sum
//! https://leetcode.com/problems/two-sum/
//! Category: HashMap

pub struct Solution;

impl Solution {
    pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
        (1..nums.len())
            .flat_map(|gap| (gap..nums.len()).map(move |right| (right - gap, right)))
            .find(|&(left, right)| nums[left] + nums[right] == target)
            .map(|(left, right)| vec![left as i32, right as i32])
            .unwrap_or_default()
    }

    pub fn two_sum_one_pass(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            if let Some(&index) = map.get(&(target - num)) {
                return vec![index as i32, i as i32];
            }
            map.insert(num, i);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(
            Solution::two_sum_brute_force(vec![2, 7, 11, 15], 9),
            vec![0, 1]
        );
        assert_eq!(Solution::two_sum_brute_force(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum_brute_force(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_one_pass() {
        assert_eq!(
            Solution::two_sum_one_pass(vec![2, 7, 11, 15], 9),
            vec![0, 1]
        );
        assert_eq!(Solution::two_sum_one_pass(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum_one_pass(vec![3, 3], 6), vec![0, 1]);
    }
}
