use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        for i in 0..arr.len() {
            if map.contains_key(&arr[i]) {
                let value = map.get_mut(&arr[i]).unwrap();
                *value += 1;
            } else {
                map.insert(arr[i], 1);
            }
        }

        let mut set: HashSet<i32> = HashSet::new();
        for (_, value) in map.iter() {
            if !set.insert(*value) {
                return false;
            }
        }

        true
    }
}