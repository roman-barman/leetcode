use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }

        let set_1: HashSet<char> = HashSet::from_iter(word1.chars());
        let set_2: HashSet<char> = HashSet::from_iter(word2.chars());    
        if set_1.difference(&set_2).count() != 0 {
            return false;
        }
        
        let map_1 = Self::create_count_map(word1);
        let map_2 = Self::create_count_map(word2);    
        for (key, value) in map_1 {
            if !map_2.contains_key(&key) {
                return false;
            }
            
            let length_1 =value.len();
            let length_2 =map_2.get(&key).unwrap().len();
            if length_1 != length_2 {
                return false;
            }
        }   
        
        true
    }

    fn create_count_map(word1: String) -> HashMap<usize, HashSet<char>> {
        let mut chars_map: HashMap<char, usize> = HashMap::new();
        for c in word1.chars() {
            if chars_map.contains_key(&c) {
                chars_map.insert(c, chars_map[&c] + 1);
            } else {
                chars_map.insert(c, 1);
            }
        }

        let mut count_map: HashMap<usize, HashSet<char>>  = HashMap::new();
        for (key, value) in chars_map {
            if count_map.contains_key(&value) {
                count_map.get_mut(&value).unwrap().insert(key);
            } else {
                let mut set = HashSet::new();
                set.insert(key);
                count_map.insert(value, set);
            }
        }
        
        count_map
    }
}