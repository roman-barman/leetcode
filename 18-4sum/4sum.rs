impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = nums.len();

        if len < 4 {
            return Vec::new();
        }

        let target = target as i64;
        let mut nums : Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        nums.sort_unstable();

        let mut result : Vec<Vec<i32>> = Vec::with_capacity(128);

        let mut i = 0;
        while i < len - 3 {
            let mut j = i + 1;
            while j < len - 2 {
                let new_target = target - nums[i] - nums[j];
                let mut low = j + 1;
                let mut high = len -1;

                while low < high {
                    if (nums[low]) + (nums[high]) < new_target {
                        low = low + 1;
                    } else if (nums[low]) + (nums[high]) > new_target {
                        high = high - 1;
                    } else {
                        result.push(vec![nums[i] as i32,nums[j] as i32,nums[low] as i32,nums[high] as i32]);
                        let temp_index_1 = low;
                        let temp_index_2 = high;

                        while low < high && nums[temp_index_1] == nums[low] {
                            low = low + 1;
                        }

                        while low < high && nums[temp_index_2] == nums[high] {
                            high = high - 1;
                        }
                    }
                }

                while j + 1 < len && nums[j] == nums[j + 1] {
                    j = j + 1;
                }
                j = j + 1;
            }

            while i + 1 < len && nums[i] == nums[i + 1] {
                i = i + 1;
            }
            i = i + 1;
        }

        result
    }
}