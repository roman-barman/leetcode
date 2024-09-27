impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len =  nums.len();
        let k = k as usize % len;

        if nums.len() < 2 || k == 0 {
            return;
        }

        let start = len - k;
        let mut temp = Vec::from_iter(nums[start..len].iter().cloned());
        temp.extend_from_slice(&nums[0..start]);
        

        nums.clear();
        nums.extend_from_slice(&temp);
    }
}