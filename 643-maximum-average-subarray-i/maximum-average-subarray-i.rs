impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
         if nums.len() == 1 {
            return nums[0] as f64;
        }
        
        let mut result = f64::MIN;
        
        for i in 0..nums.len() - (k - 1) as usize {
            let mut sum = 0;
            for j in i..i + k as usize {
                sum += nums[j];
            }
            let average = sum as f64 / k as f64;
            
            if average > result {
                result = average;
            }
        }
        
        result
    }
}