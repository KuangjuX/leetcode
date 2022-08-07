use std::cmp;
impl Solution {
    pub fn get_max(i: usize, j: usize, height: &Vec<i32>) -> i32 {
        let mut max = height[i];
        for n in i..=j {
            max = cmp::max(max, height[n]);
        }
        max
    }

    // 直接按列求
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

    // 动态规划
    pub fn trap2(height: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut left_max = vec![0;height.len()];
        let mut right_max = vec![0;height.len()];
        left_max[0] = height[0];
        right_max[height.len() - 1] = height[height.len() - 1];
        for col in 1..height.len() - 1 {
            left_max[col] = cmp::max(left_max[col - 1], height[col - 1]);
        }
        for col in (1..height.len() - 1).rev() {
            right_max[col] = cmp::max(right_max[col + 1], height[col + 1]);
        }
        for col in 1..height.len() - 1 {
            let mut increase = cmp::min(left_max[col], right_max[col]) - height[col];
            if increase < 0 {
                increase = 0;
            }
            sum += increase;
        }
        sum
    }
}