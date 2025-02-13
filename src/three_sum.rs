//! Solution for LeetCode problem #15: 3Sum
//! https://leetcode.com/problems/3sum/

pub struct Solution;

impl Solution {
    pub fn three_sum_imperative(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        if nums[0] > 0 {
            return vec![];
        }

        let mut res = Vec::with_capacity(nums.len() / 3);
        let length = nums.len();
        for i in 0..length {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = length - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                if sum < 0 {
                    left += 1;
                } else if sum > 0 {
                    right -= 1;
                } else {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                        continue;
                    }
                    while nums[right] == nums[right + 1] && left < right {
                        right -= 1;
                        continue;
                    }
                }
            }
        }
        res
    }

    pub fn three_sum_functional(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        if nums[0] > 0 {
            return vec![];
        }

        (0..nums.len())
            .filter(|&i| i == 0 || nums[i] != nums[i - 1])
            .flat_map(|i| Self::two_sum(&nums, i))
            .collect()
    }

    fn two_sum(nums: &[i32], i: usize) -> Vec<Vec<i32>> {
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        let mut res = Vec::new();

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            match sum.cmp(&0) {
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
                std::cmp::Ordering::Equal => {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                        continue;
                    }

                    while nums[right] == nums[right + 1] && left < right {
                        right -= 1;
                        continue;
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imperative() {
        assert_eq!(
            Solution::three_sum_imperative(vec![-1, 0, 1, 2, -1, -4]),
            [[-1, -1, 2], [-1, 0, 1]]
        );
        assert_eq!(
            Solution::three_sum_imperative(vec![0, 1, 1]),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(Solution::three_sum_imperative(vec![0, 0, 0]), [[0, 0, 0]]);
    }

    #[test]
    fn test_functional() {
        assert_eq!(
            Solution::three_sum_functional(vec![-1, 0, 1, 2, -1, -4]),
            [[-1, -1, 2], [-1, 0, 1]]
        );
        assert_eq!(
            Solution::three_sum_functional(vec![0, 1, 1]),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(Solution::three_sum_functional(vec![0, 0, 0]), [[0, 0, 0]]);
    }
}
