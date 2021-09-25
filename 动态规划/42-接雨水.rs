use std::cmp;
impl Solution {
    pub fn get_max(i: usize, j: usize, height: &Vec<i32>) -> i32 {
        let mut max = height[i];
        for n in i..=j {
            max = cmp::max(max, height[n]);
        }
        max
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        let mut sum = 0;
        for col in 1..height.len() - 1 {
            let left_max = Self::get_max(0, col - 1, &height);
            let right_max = Self::get_max(col + 1, height.len() - 1, &height);
            let mut increase = cmp::min(left_max, right_max) - height[col];
            if increase < 0 {
                increase = 0;
            }
            
            sum += increase;
        }
        sum
    }
}