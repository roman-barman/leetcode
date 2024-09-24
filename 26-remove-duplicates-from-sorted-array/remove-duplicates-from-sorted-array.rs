impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut result = 1;

        for index in 1..nums.len() {
            if nums[index] != nums[index - 1] {
                nums[result] = nums[index];
                result = result + 1;
            }
        }

        result as i32
    }
}