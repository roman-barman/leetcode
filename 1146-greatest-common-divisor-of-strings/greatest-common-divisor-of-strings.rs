impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if format!("{}{}", str1, str2) != format!("{}{}", str2, str1) {
            return "".to_string();
        }
        
        let str1_len = str1.len();
        let str2_len = str2.len();

        if str1_len == str2_len {
            return str1;
        }

        if str1.len() > str2.len() {
            Self::gcd_of_strings(String::from(&str1[str2_len..]), str2)
        } else {
            Self::gcd_of_strings(str1, String::from(&str2[str1_len..]))
        }
    }
}