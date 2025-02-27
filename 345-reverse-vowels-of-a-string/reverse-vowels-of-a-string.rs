impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }

        let mut chars: Vec<_> = s.chars().collect();

        let mut left_index = 0;
        let mut right_index = s.len() - 1;

        while left_index < right_index {
            while !Self::is_vowel(chars[left_index]) && left_index < right_index  {
                left_index += 1;
            }

            while !Self::is_vowel(chars[right_index]) && left_index < right_index  {
                right_index -= 1;
            }

            if left_index >= right_index {
                break;
            }
            
            chars.swap(left_index, right_index);
            left_index += 1;
            right_index -= 1;
        }

        chars.into_iter().collect()
    }

    fn is_vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' 
            || c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U'
    }
}