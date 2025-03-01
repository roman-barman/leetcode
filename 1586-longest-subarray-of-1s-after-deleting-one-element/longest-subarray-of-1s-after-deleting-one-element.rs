impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        nums
            .split(|&el| el == 0)
            .map(|ones_slice| ones_slice.len() as i32)
            .scan(-1, |prev, curr| {
                let output = *prev + curr;
                *prev = curr;
                Some(output)
            })
            .max()
            .unwrap_or(0)
    }
}