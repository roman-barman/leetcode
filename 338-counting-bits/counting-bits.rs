impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut result : Vec<i32> = vec![0;n + 1];
        result[0] = 0;

        for i in 1..=n {
            result[i] = result[i >> 1] + (i & 1) as i32;
        }

        result
    }
}