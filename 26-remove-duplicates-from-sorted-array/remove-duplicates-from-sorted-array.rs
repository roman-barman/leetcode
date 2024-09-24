impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();

        if len == 0 {
            return 0;
        }

        if len == 1 {
            return 1;
        }

        let mut index_1 : usize = 0;
        let mut index_2 : usize = 0;
        let mut result = 1;

        loop {
            if index_1 == len - 1 {
                break;
            }

            let val = nums[index_1];

            index_1 = index_1 + 1;
            index_2 = index_2 + 1;

            while val == nums[index_1] && index_1 != len - 1 {
                index_1 = index_1 + 1;
            }

            if index_1 == len - 1 &&  val == nums[index_1] {
                break;
            }

            nums[index_2] = nums[index_1];
            result = result + 1;
        }

        result
    }
}