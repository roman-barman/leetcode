impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len =  nums.len();
        let k = k as usize % len;

        if nums.len() < 2 || k == 0 {
            return;
        }

        let mut temp = Vec::with_capacity(nums.len());

        let start = len - k;
        for i in start..len {
            temp.push(nums[i]);
        }

        for i in 0..start {
            temp.push(nums[i]);
        }

        nums.clear();
        nums.extend_from_slice(&temp);
    }
}