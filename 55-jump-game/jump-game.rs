impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 0 {
            return false;
        }

        let last_index = nums.len() - 1;
        let mut is_visited = vec![false;nums.len()];
        let mut max_index = 0;

        while max_index < last_index {
            if is_visited[max_index] && max_index == 0 {
                return false;
            }
            if is_visited[max_index] {
                max_index = max_index - 1;
                continue;
            } else {
                is_visited[max_index] = true;
            }

            max_index = max_index + nums[max_index] as usize;
        }

        true
    }
}