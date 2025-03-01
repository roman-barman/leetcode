use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let (mut array_1, mut array_2) = ([0; 26], [0; 26]);
        for c in word1.chars() { array_1[c as usize - 0x61] += 1; };
        for c in word2.chars() { array_2[c as usize - 0x61] += 1; };
        
        for i in 0..26 {
            if (array_1[i] == 0 && array_2[i] != 0) || (array_1[i] != 0 && array_2[i] == 0) {
                return false;
            }
        }
        
        array_1.sort();
        array_2.sort();

        array_1 == array_2
    }
}