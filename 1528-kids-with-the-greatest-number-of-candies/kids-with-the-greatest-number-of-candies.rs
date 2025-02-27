impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max = candies.iter().max().unwrap();
        let mut result: Vec<bool> = vec![false; candies.len()];
        
        for (index, value) in candies.iter().enumerate() {
            if *value + extra_candies >= *max {
                result[index] = true;
            }
        }
        
        result
    }
}