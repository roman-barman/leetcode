impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
         if s.is_empty() {
            return true;
        }
        
        if s.len() > t.len() {
            return false;
        }
        
        if s.len() == t.len() {
            return s == t;
        }
        
        let mut current_index = 0;
        
        for i in 0..t.len() {        
            if s.get(current_index..current_index + 1).unwrap() == t.get(i..i + 1).unwrap() {
                current_index += 1;
            }

            if current_index >= s.len() {
                return true;
            }
        }
        
        false
    }
}