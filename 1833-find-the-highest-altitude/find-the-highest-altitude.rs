impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut altitude = 0;
        
        for i in 0..gain.len() {
            altitude += gain[i];
            
            if altitude > max {
                max = altitude;
            }
        }
        
        max
    }
}