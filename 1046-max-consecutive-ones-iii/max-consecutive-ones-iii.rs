impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut max_length = 0;
        let mut zero_count = 0;

        for right in 0..nums.len() {
            if nums[right] == 0 {
                zero_count += 1;
            }

            while zero_count > k {
                if nums[left] == 0 {
                    zero_count -= 1;
                }
                left += 1;
            }

            let length = right - left + 1;
            max_length = if max_length > length {
                max_length
            } else {
                length
            };
        }

        max_length as i32
    }
}