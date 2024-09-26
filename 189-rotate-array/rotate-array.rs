impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.len() < 2 {
            return;
        }

        let last_index = nums.len() - 1;

        for _ in 0..(k as usize % nums.len()) {
            let temp = nums[last_index];

            let mut j = last_index - 1;

            loop {
                nums[j + 1] = nums[j];

                if j == 0 {
                    break;
                }

                j = j - 1;
            }

            nums[0] = temp;
        }
    }
}