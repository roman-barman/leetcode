impl Solution {
    pub fn reverse_words(s: String) -> String {
        let s: String = s.trim().split_whitespace().rev().map(|x| format!("{} ", x)).collect();
        s.trim().to_string()
    }
}