//! Solution for LeetCode problem #322: Coin Change
//! https://leetcode.com/problems/coin-change/

pub struct Solution;

impl Solution {
    pub fn coin_change_imperative(coins: Vec<i32>, amount: i32) -> i32 {
        let amt = amount as usize;
        let mut dp = vec![amount + 1; amt + 1];
        dp[0] = 0;
        for i in 1..=amt {
            for &coin in &coins {
                let coin = coin as usize;
                if i >= coin {
                    dp[i] = dp[i].min(1 + dp[i - coin]);
                }
            }
        }
        if dp[amt] > amount {
            -1
        } else {
            dp[amt]
        }
    }

    pub fn coin_change_functional(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let dp: Vec<Option<i32>> =
            (0..=amount).fold(Vec::with_capacity(amount + 1), |mut dp, i| {
                let entry = if i == 0 {
                    Some(0)
                } else {
                    coins
                        .iter()
                        .filter_map(|&coin| {
                            let coin = coin as usize;
                            if i >= coin {
                                dp[i - coin].map(|cur| 1 + cur)
                            } else {
                                None
                            }
                        })
                        .min()
                };

                dp.push(entry);
                dp
            });

        dp[amount].unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imperative() {
        assert_eq!(Solution::coin_change_imperative(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change_imperative(vec![2], 3), -1);
        assert_eq!(Solution::coin_change_imperative(vec![1], 0), 0);
    }

    #[test]
    fn test_functional() {
        assert_eq!(Solution::coin_change_functional(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change_functional(vec![2], 3), -1);
        assert_eq!(Solution::coin_change_functional(vec![1], 0), 0);
    }
}
