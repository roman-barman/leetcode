use std::cmp;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let last_index = len - 1;

        let mut temp = vec![0;len];

        for i in 0..last_index {
            let count = temp[i] + 1;
            let max_index = cmp::min(i+nums[i] as usize, last_index);

            for j in i..=max_index {
                if temp[j] == 0 {
                    temp[j] = count;
                } else if temp[j] > count {
                    temp[j] = count;
                }
            }
        }
        temp[last_index]
    }
}