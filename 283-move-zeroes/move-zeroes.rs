impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zero_index = usize::max_value();
        for i in 0..nums.len() {
            if nums[i] == 0 {
                zero_index = i;
                break;
            }
        }
        
        if zero_index == usize::max_value() {
            return;
        }
        
        for i in zero_index+1..nums.len() {
            if nums[i] != 0 {
                nums.swap(i, zero_index);
                zero_index += 1;
            }
        }
    }
}