impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        if word1.is_empty() {
            return word2;
        }

        if word2.is_empty() {
            return word1;
        }

        let mut result = String::new();
        result.push_str(&word1[0..1]);
        result.push_str(&word2[0..1]);

        let min_size = if word1.len() < word2.len() {
            word1.len()
        } else {
            word2.len()
        };

        for index in 1..min_size {
            result.push_str(&word1[index..index + 1]);
            result.push_str(&word2[index..index + 1]);
        }

        if word1.len() > min_size {
            result.push_str(&word1[min_size..]);
        }

        if word2.len() > min_size {
            result.push_str(&word2[min_size..]);
        }

        result
    }
}