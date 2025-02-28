impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
         let mut windows_sum = nums.iter().take(k as usize).sum::<i32>();
        let mut max_sum = windows_sum;

        for i in k as usize..nums.len() {
            windows_sum += nums[i] - nums[i - k as usize];
            max_sum = if max_sum > windows_sum { max_sum } else { windows_sum };
        }

        max_sum as f64 / k as f64
    }
}