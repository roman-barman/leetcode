use std::collections::HashSet;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();

        if len < 4 {
            return Vec::new();
        }

        let target = target as i64;
        let mut nums : Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        nums.sort_unstable();

        let mut set : HashSet<Vec<i32>> = HashSet::with_capacity(256);

        for i in 0..len - 3 {
            for j in i + 1..len - 2 {
                let new_target = target - nums[i] - nums[j];
                let mut low = j + 1;
                let mut high = len -1;

                while low < high {
                    if (nums[low]) + (nums[high]) < new_target {
                        low = low + 1;
                    } else if (nums[low]) + (nums[high]) > new_target {
                        high = high - 1;
                    } else {
                        set.insert(vec![nums[i] as i32,nums[j] as i32,nums[low] as i32,nums[high] as i32]);
                        low = low + 1;
                        high = high - 1;
                    }
                }
            }
        }

        let mut result : Vec<Vec<i32>> = Vec::with_capacity(set.len());

        for v in set {
            result.push(v);
        }

        result
    }
}