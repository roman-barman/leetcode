impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let half = len / 2;

        if len == 1 {
            return nums[0];
        }

        for i in 0..len {
            let mut count = 1;
            let val = nums[i];

            for j in i + 1..len {
                if nums[j] == val {
                    count = count + 1;
                }

                if count > half {
                    return val;
                }
            }
        }

        0
    }
}