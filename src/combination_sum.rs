//! Solution for LeetCode problem #39: Combination Sum
//! https://leetcode.com/problems/combination-sum/

pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::dfs(&candidates, target, 0, &mut vec![], &mut res);
        res
    }

    fn dfs(
        candidates: &Vec<i32>,
        target: i32,
        index: usize,
        cur: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            res.push(cur.clone());
            return;
        }
        if index >= candidates.len() || target < 0 {
            return;
        }
        cur.push(candidates[index]);
        Self::dfs(candidates, target - candidates[index], index, cur, res);
        cur.pop();
        Self::dfs(candidates, target, index + 1, cur, res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2], 1),
            Vec::<Vec<i32>>::new()
        );
    }
}
