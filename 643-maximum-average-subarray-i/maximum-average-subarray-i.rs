impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
         if nums.len() == 1 {
            return nums[0] as f64;
        }
        
        let mut max_sum = i32::MIN;
        
        for i in 0..nums.len() - (k - 1)  as usize {
            let mut sum = 0;
            for j in i..i + k as usize {
                sum += nums[j];
            }
            
            if sum > max_sum {
                max_sum = sum;
            }
        }
        
        max_sum as f64 / k as f64
    }
}