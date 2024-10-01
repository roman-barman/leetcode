impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        
        let middle = (nums.len() + 1) / 2;
        let mut j = middle - 1;
        let mut k = nums.len() - 1;

        for i in 0..nums.len() {
            if i % 2 == 0 {
                nums[i] = sorted_nums[j];

                if j != 0 {
                    j = j - 1;
                }
            } else {
                nums[i] = sorted_nums[k];
                k = k - 1;
            }
        }
    }
}