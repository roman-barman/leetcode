impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let mut nums_sorted = vec![];
        let mut result = vec![];
        nums.iter().rev().for_each(|&n| {
            let pos = nums_sorted
                .binary_search_by(|probe: &i32| probe.cmp(&(2 * n - 1)))
                .unwrap_or_else(|x| x);
            result.push(pos as i32);
            nums_sorted.insert(pos, 2 * n);
        });
        result.reverse();
        result
    }
}