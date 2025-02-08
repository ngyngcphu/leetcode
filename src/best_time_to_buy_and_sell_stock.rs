//! Solution for LeetCode problem #121: Best Time to Buy and Sell Stock
//! https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
//! Category: Sliding Window (Two Pointers)

pub struct Solution;

impl Solution {
    pub fn max_profits_imperative(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = std::i32::MAX;

        for price in prices {
            min_price = min_price.min(price);
            max_profit = max_profit.max(price - min_price);
        }

        max_profit
    }

    pub fn max_profits_functional(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .fold((0, std::i32::MAX), |(max_profit, min_price), &price| {
                (max_profit.max(price - min_price), min_price.min(price))
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_best_time_to_buy_and_sell_stock_imperative() {
        assert_eq!(Solution::max_profits_imperative(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profits_imperative(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_best_time_to_buy_and_sell_stock_functional() {
        assert_eq!(Solution::max_profits_functional(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profits_functional(vec![7, 6, 4, 3, 1]), 0);
    }
}
