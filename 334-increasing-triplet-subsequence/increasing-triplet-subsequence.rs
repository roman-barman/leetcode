impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }
        
        let mut first = i32::MAX;
        let mut second = i32::MAX;
        
        for value in nums {
            if value <= first {
                first = value;
            } else if value <= second {
                second = value;
            } else {
                return true;
            }
        }
        
        false
    }
}