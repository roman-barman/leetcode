impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        if chars.len() < 2 {
            return chars.len() as i32;
        }
        
        let mut count = 1;
        let mut result = 0;
        let mut last_index = 0usize;
        
        for i in 0..chars.len() - 1 {
            if chars[i] == chars[i + 1] {
                count += 1;
            } else {
                chars[last_index] = chars[i];
                last_index += 1;
                result += 1;
                if count >= 2 {
                    result += Self::add_count(count, chars, &mut last_index);
                }
                count = 1;
            }
        }

        chars[last_index] = chars[chars.len() - 1];
        last_index += 1;
        result += 1;
        
        if count >= 2 {
            result += Self::add_count(count, chars, &mut last_index);
        }
        
        chars.truncate(last_index);
        
        result
    }

    fn add_count(count: i32, chars: &mut Vec<char>, start_index: &mut usize) -> i32 {
        let mut divider = 10;
        let mut count = count;

        while divider <= count {
            divider *= 10;
        }
        
        let mut result = 0;
        
        while divider > 10 {
            divider /= 10;
            let digit = count / divider;
            chars[*start_index] = char::from_digit(digit as u32, 10).unwrap();
            *start_index += 1;
            count = count % divider;
            result += 1;
        }

        chars[*start_index] = char::from_digit(count as u32, 10).unwrap();
        *start_index += 1;
        result += 1;
        
        result
    }
}