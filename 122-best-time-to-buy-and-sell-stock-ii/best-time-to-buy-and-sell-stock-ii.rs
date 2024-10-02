impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profits = Vec::with_capacity(prices.len());
        let mut buy = prices[0];
        for i in 1..prices.len() {
            if prices[i] < buy {
                buy = prices[i];
            } else if prices[i] - buy > 0 {
                profits.push(prices[i] - buy);
                buy = prices[i];
            }
        }
        profits.iter().sum()
    }
}