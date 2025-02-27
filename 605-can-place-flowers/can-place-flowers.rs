impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0 {
            return true;
        }
        
        let mut previous = 0;    
        let mut n = n;
        
        for i in 0..flowerbed.len() {
            if flowerbed[i] == 1 {
                previous = 1;
                continue;
            }
            
            let next = if i == flowerbed.len() - 1 { 0 } else { flowerbed[i + 1] };
            
            if previous == 0 && next == 0 {
                n = n - 1;
                previous = 1;
                if n == 0 {
                    return true
                }
            } else { 
                previous = 0;
            }       
        }    
        
        false
    }
}