use std::cmp;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let last_index = len - 1;
        let mut count = 0;
        let mut end = 0;
        let mut range = 0;

        for i in 0..last_index {
            range = cmp::max(range, i + nums[i] as usize);

            if range >= last_index {
                count = count + 1;
                break;
            }

            if end == i {
                end = range;
                count = count + 1;
            }
        }
        count
    }
}