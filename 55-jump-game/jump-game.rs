impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 0 {
            return false;
        }

        let last_index = nums.len() - 1;

        let mut jumps = Vec::with_capacity(nums.len());
        let mut is_visited = vec![false;nums.len()];
        jumps.push(0);

        while !jumps.is_empty() {
            let current_index = jumps.pop().unwrap();
            is_visited[current_index] = true;
            let max_index = current_index + nums[current_index] as usize;
            if max_index >= last_index {
                return true
            }

            if max_index > current_index {
                for i in current_index + 1..=max_index {
                    if !is_visited[i] {
                        jumps.push(i);
                    }
                }
            }
        }

        false
    }
}