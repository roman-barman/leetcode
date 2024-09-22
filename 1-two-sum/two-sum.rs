impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(2);
        let mut map : std::collections::HashMap<i32, Vec<usize>> = std::collections::HashMap::new();

        for (index, value) in nums.iter().enumerate() {
            if map.contains_key(&value) {
                map.get_mut(&value).unwrap().push(index);
            } else {
                let mut vec = Vec::new();
                vec.push(index);
                map.insert(*value, vec);

            }
        }

        for (index, value) in nums.iter().enumerate() {
            let second_value = target - *value;

            if map.contains_key(&second_value) {
                for second_index in map.get(&second_value).unwrap() {
                    if *second_index != index {
                        result.push(index as i32);
                        result.push(*second_index as i32);

                        return result;
                    }
                }
            }
        }

        result
    }
}