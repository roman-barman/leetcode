use std::ops::Index;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let mut n = x;
        let mut reversed = 0;

        while n != 0 {
            reversed = reversed * 10 + n % 10;
            n /= 10;
        }

        x == reversed
    }
}