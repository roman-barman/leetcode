use std::ops::Index;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut div = x;
        let mut calc = Vec::new();

        while div > 9 {
            calc.push(div % 10);
            div = div / 10;
        }
        calc.push(div);

        let mut index = 0;
        let mut last_index = calc.len() - 1;
        let middle = calc.len() / 2;

        while index <= middle  {
            if calc.index(index) != calc.index(last_index) {
                return false;
            }

            index = index + 1;
            last_index = last_index - 1;
        }

        true
    }
}