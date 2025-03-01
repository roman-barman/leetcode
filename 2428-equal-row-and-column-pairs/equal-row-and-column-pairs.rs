use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut hash = HashMap::<&[i32], i32>::new();
        for row in grid.iter() {
            *hash.entry(row).or_insert(0) += 1;
        }
        let mut ans = 0;
        let mut v = vec![0; grid.len()];
        for j in 0..grid.len() {
            for i in 0..grid.len() {
                v[i] = grid[i][j];
            }
            if let Some(&x) = hash.get(&v[..]) {
                ans += x;
            }
        }
        ans
    }
}