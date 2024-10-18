impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0;nums.len()];

        for i in 0..nums.len() {
            let mut count = 0;
            for j in i + 1 ..nums.len() {
                if nums[i] > nums[j] {
                    count = count + 1;
                }
            }

            result[i] = count;
        }

        result
    }
}