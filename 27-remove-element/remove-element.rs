impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index_1 : usize = 0;
        let mut index_2 : usize = 0;
        let mut result = 0;
        let len = nums.len();

        if len == 0 {
            return 0;
        }

        loop {
            if nums[index_1] != val {
                index_1 = index_1 + 1;
                index_2 = index_2 + 1;
                result = result + 1;
            } else {
                index_1 = index_1 + 1;
            }

            if index_1 == len {
                break;
            }

            if index_1 != index_2 {
                nums[index_2] = nums[index_1];
            }
        }

        result
    }
}