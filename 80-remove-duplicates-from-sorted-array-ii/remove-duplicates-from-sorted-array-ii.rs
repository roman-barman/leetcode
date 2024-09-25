impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut result = 1;

        for index in 1..nums.len() {
            if result == 1 || nums[index] != nums[result - 2] {
                nums[result] = nums[index];
                result = result + 1;
            }
        }

        result as i32
    }
}