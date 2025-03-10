impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut k = k;
        let len = nums.len();
        let mut start = 0;
        let mut end = 0;
        while end < len {
            if nums[end] == 0 {
                k -= 1;
            }

            if k < 0 {
                if nums[start] == 0 { 
                    k += 1;
                }
                start += 1;
            }
            
            end += 1;
        }

        (end - start) as i32
    }
}