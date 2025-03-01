impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut sum_left = vec![0];
        let mut sum_right = vec![nums.iter().skip(1).sum::<i32>()];
        
        for i in 1..nums.len() {
            sum_left.push(sum_left[i - 1] + nums[i - 1]);
            sum_right.push(sum_right[i - 1] - nums[i]);
        }
        
        for i in 0..nums.len() {
            if sum_left[i] == sum_right[i] {
                return i as i32;
            }
        }
        
        -1
    }
}