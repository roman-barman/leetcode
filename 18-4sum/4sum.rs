use std::collections::HashSet;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();

        if len < 4 {
            return Vec::new();
        }

        let mut nums = nums.clone();
        nums.sort();

        let mut set : HashSet<Vec<i32>> = HashSet::new();

        for i in 0..len - 3 {
            for j in i + 1..len - 2 {
                let new_target = target as i64 - nums[i] as i64 - nums[j] as i64;
                let mut low = j + 1;
                let mut high = len -1;

                while low < high {
                    if (nums[low] as i64) + (nums[high] as i64) < new_target {
                        low = low + 1;
                    } else if (nums[low] as i64) + (nums[high] as i64) > new_target {
                        high = high - 1;
                    } else {
                        set.insert(vec![nums[i],nums[j],nums[low],nums[high]]);
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