impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let len = temperatures.len();
        let mut result = vec![0i32; len];
        let mut stack = Vec::with_capacity(len);

        for i in 0..len {
            while stack.len() > 0 &&  temperatures[*(stack.last().unwrap())] < temperatures[i] {
                let index = stack.pop().unwrap();
                result[index] = (i - index) as i32;
            }

            stack.push(i);
        }

        result
    }
}