impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(asteroids.len());  
    
        for asteroid in asteroids {
            if result.is_empty() {
                result.push(asteroid);
                continue;
            }
            
            let mut need_add = true;
            while !result.is_empty() {
                let last = result[result.len() - 1];
                
                if last > 0 && asteroid < 0 {
                    let abs_asteroid = asteroid.abs();
                    
                    if last < abs_asteroid {
                        result.pop();
                    } else if last == abs_asteroid {
                        result.pop();
                        need_add = false;
                        break;
                    } else {
                        need_add = false;
                        break;
                    }
                } else {
                    break;
                }
            }
            if need_add {
                result.push(asteroid);
            }
        }

        result
    }
}