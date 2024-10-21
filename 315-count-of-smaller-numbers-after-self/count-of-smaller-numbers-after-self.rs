impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        let mut nums = nums.into_iter().enumerate().collect();
        Self::merge_sort(&mut nums, &mut ans);
        ans
    }

    pub fn merge_sort(nums: &Vec<(usize, i32)>, ans: &mut Vec<i32>) -> Vec<(usize, i32)> {
        if nums.len() < 2 {
            return nums.clone();
        }

        let mut left = Vec::new();
        let mut right = Vec::new();

        let middle = nums.len() / 2;

        for i in 0..middle {
            left.push(nums[i]);
        }

        for i in middle..nums.len() {
            right.push(nums[i]);
        }

        let sorted_left = Self::merge_sort(&left, ans);
        let sorted_right = Self::merge_sort(&right, ans);

        Self::merge(&sorted_left, &sorted_right, ans)
    }

    pub fn merge(left: &Vec<(usize, i32)>, right: &Vec<(usize, i32)>, ans: &mut Vec<i32>) -> Vec<(usize, i32)> {
        let mut result = Vec::with_capacity(left.len() + right.len());
        let mut i = 0;
        let mut j = 0;
        let mut count = 0;

        while i < left.len() && j < right.len() {
            if left[i].1 > right[j].1 {
                result.push(right[j]);
                j += 1;
                count += 1;
            } else {
                result.push(left[i]);
                ans[left[i].0] += count;
                i += 1;
            }
        }

        while i < left.len() {
            ans[left[i].0] += right.len() as i32;
            result.push(left[i]);
            i += 1;
        }

        while j < right.len() {
            result.push(right[j]);
            j += 1;
        }

        result
    }
}