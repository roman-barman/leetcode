impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut result = 1;
        let mut dublicate_count = 0;

        for index in 1..nums.len() {
            if nums[index] != nums[index - 1] {
                nums[result] = nums[index];
                dublicate_count = 0;
                result = result + 1;
            } else if dublicate_count < 1 {
                nums[result] = nums[index];
                dublicate_count = dublicate_count + 1;
                result = result + 1;
            } else {
                dublicate_count = dublicate_count + 1;
            }
        }

        result as i32
    }
}