use std::cmp::min;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_index = 0usize;
        let mut right_index = height.len() - 1;
        
        let mut max_area = 0;
        
        while left_index <= right_index {
            let min_height = min(height[left_index], height[right_index]);
            let length = (right_index - left_index) as i32;
            
            let area = min_height * length;
            
            if area > max_area {
                max_area = area;
            }
            
            if height[left_index] <= height[right_index] {
                left_index += 1;
            } else {
                right_index -= 1;
            }
        }
        
        max_area
    }
}

