impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() < 2 {
            return chars.len() as i32;
        }
        
        let mut count = 1;
        let mut result = 0;
        let mut last_index = 0 as usize;
        
        for i in 0..chars.len() - 1 {
            if chars[i] == chars[i + 1] {
                count += 1;
            } else {
                chars[last_index] = chars[i];
                last_index += 1;
                result += 1;
                if count >= 2 {                
                    let count_chars = count.to_string().chars().collect::<Vec<char>>();
                    
                    for char in count_chars.iter() {
                        chars[last_index] = *char;
                        last_index += 1;
                    }                
                    result += count_chars.len() as i32;
                }
                count = 1;
            }
        }

        chars[last_index] = chars[chars.len() - 1];
        last_index += 1;
        result += 1;
        
        if count >= 2 {
            let count_chars = count.to_string().chars().collect::<Vec<char>>();

            for char in count_chars.iter() {
                chars[last_index] = *char;
                last_index += 1;
            }
            result += count_chars.len() as i32;
        }
        
        chars.truncate(last_index);
        
        result
    }
}