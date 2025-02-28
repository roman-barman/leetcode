impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut window_size = 0;
        let k = k as usize;
        
        for i in 0..k {
            if Self::is_vowel(&s[i..i + 1]) {
                window_size += 1;
            }
        }

        let mut max_size = window_size;
        
        for i in k..s.len() {        
            window_size += if Self::is_vowel(&s[i..i + 1]) { 1 } else { 0 };
            window_size += if Self::is_vowel(&s[i-k..i - k + 1]) { -1 } else { 0 };
            
            if window_size > max_size {
                max_size = window_size;
            }
        }

        max_size
    }

    fn is_vowel(s: &str) -> bool {
        s == "a" || s == "e" || s == "i" || s == "o" || s == "u"
    }
}