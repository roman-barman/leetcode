use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let hash_set_1: HashSet<i32> = HashSet::from_iter(nums1);
        let hash_set_2: HashSet<i32> = HashSet::from_iter(nums2);    
        vec![hash_set_1.difference(&hash_set_2).cloned().collect(), hash_set_2.difference(&hash_set_1).cloned().collect()]
    }
}