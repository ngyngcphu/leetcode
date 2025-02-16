//! Solution for LeetCode problem #300: Longest Increasing Subsequence
//! https://leetcode.com/problems/longest-increasing-subsequence/

pub struct Solution;

impl Solution {
    //Dynamic Programming: O(n^2)
    pub fn length_of_lis_dynamic_programming_imperative(nums: Vec<i32>) -> i32 {
        let length = nums.len();
        let mut dp = vec![1; length];
        for i in (0..length).rev() {
            for j in (i + 1)..length {
                if nums[i] < nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        dp.into_iter().max().unwrap_or(1)
    }

    pub fn length_of_lis_dynamic_programming_functional(nums: Vec<i32>) -> i32 {
        nums.iter()
            .rev()
            .fold(Vec::with_capacity(nums.len()), |mut dp, &num| {
                let best = dp
                    .iter()
                    .filter_map(|&(i, lis)| if num < i { Some(lis + 1) } else { None })
                    .max()
                    .unwrap_or(1);
                dp.push((num, best));
                dp
            })
            .into_iter()
            .map(|(_, lis)| lis)
            .max()
            .unwrap_or(1)
    }

    //Binary Search: O(nlogn)
    pub fn length_of_lis_binary_search_imperative(nums: Vec<i32>) -> i32 {
        let mut lis = Vec::new();
        for &num in &nums {
            if lis.is_empty() || num > *lis.last().unwrap() {
                lis.push(num);
            } else {
                let idx = Self::lower_bound(&lis, num);
                lis[idx] = num;
            }
        }
        lis.len() as i32
    }

    fn lower_bound(lis: &[i32], target: i32) -> usize {
        let mut left = 0;
        let mut right = lis.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if lis[mid] == target {
                return mid;
            } else if lis[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }

    pub fn length_of_lis_binary_search_functional(nums: Vec<i32>) -> i32 {
        nums.iter()
            .fold(Vec::new(), |mut lis, &i| {
                if let Some(&last) = lis.last() {
                    if i > last {
                        lis.push(i);
                    } else if i == last {
                        return lis;
                    } else {
                        let idx = lis.binary_search(&i).unwrap_or_else(|x| x);
                        lis[idx] = i
                    }
                } else {
                    lis.push(i);
                }
                lis
            })
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_programming_imperative() {
        assert_eq!(
            Solution::length_of_lis_dynamic_programming_imperative(vec![
                10, 9, 2, 5, 3, 7, 101, 18
            ]),
            4
        );
        assert_eq!(
            Solution::length_of_lis_dynamic_programming_imperative(vec![0, 1, 0, 3, 2, 3]),
            4
        );
        assert_eq!(
            Solution::length_of_lis_dynamic_programming_imperative(vec![7, 7, 7, 7, 7, 7, 7]),
            1
        );
    }

    #[test]
    fn test_dynamic_programming_functional() {
        assert_eq!(
            Solution::length_of_lis_dynamic_programming_functional(vec![
                10, 9, 2, 5, 3, 7, 101, 18
            ]),
            4
        );
        assert_eq!(
            Solution::length_of_lis_dynamic_programming_functional(vec![0, 1, 0, 3, 2, 3]),
            4
        );
        assert_eq!(
            Solution::length_of_lis_dynamic_programming_functional(vec![7, 7, 7, 7, 7, 7, 7]),
            1
        );
    }

    #[test]
    fn test_binary_search_imperative() {
        assert_eq!(
            Solution::length_of_lis_binary_search_imperative(vec![10, 9, 2, 5, 3, 7, 101, 18]),
            4
        );
        assert_eq!(
            Solution::length_of_lis_binary_search_imperative(vec![0, 1, 0, 3, 2, 3]),
            4
        );
        assert_eq!(
            Solution::length_of_lis_binary_search_imperative(vec![7, 7, 7, 7, 7, 7, 7]),
            1
        );
    }

    #[test]
    fn test_binary_search_functional() {
        assert_eq!(
            Solution::length_of_lis_binary_search_functional(vec![10, 9, 2, 5, 3, 7, 101, 18]),
            4
        );
        assert_eq!(
            Solution::length_of_lis_binary_search_functional(vec![0, 1, 0, 3, 2, 3]),
            4
        );
        assert_eq!(
            Solution::length_of_lis_binary_search_functional(vec![7, 7, 7, 7, 7, 7, 7]),
            1
        );
    }
}
