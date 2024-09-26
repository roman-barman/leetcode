impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut value = 0;

        for i in 0..nums.len() {
            let current_value = nums[i];

            if current_value == value {
                count = count + 1;
            } else if count != 0 {
                count = count - 1;
            } else {
                value = current_value;
            }
        }

        value
    }
}