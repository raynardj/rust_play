// solution for this problem:
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown

use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn recur_profit(prices: &Vec<i32>, idx: usize, stock:i32) -> i32 {
        // stock = -1 means stock holding no stock
        if idx >= prices.len() {
            return 0;
        }
        // if Self::cache contains the tuple key
        let mut profit: i32 = 0;
        if stock >= 0 {
            // we have stock in hand
            profit = max(
                Self::recur_profit(&prices, idx + 2, -1) + prices[idx] - stock, // sell now, realize some profit, skip a day
                Self::recur_profit(&prices, idx + 1, stock) // maybe sell later
            ); 
            // insert into cache
            profit
        } else {
            // we don't have stock in hand
            profit = max(
                Self::recur_profit(&prices, idx + 1, prices[idx]), // buy now
                Self::recur_profit(&prices, idx + 1, stock) // maybe buy later
            );
            profit
        }
    }
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        return Self::recur_profit(&prices, 0, -1);
    }
}