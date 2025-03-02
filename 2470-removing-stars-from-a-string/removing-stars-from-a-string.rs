impl Solution {
    pub fn remove_stars(s: String) -> String {
        s.chars().fold(String::new(), |mut result, c| { 
            match c {
                '*' => { result.pop(); },
                _ => result.push(c),
            }; 
            result
        })
    }
}