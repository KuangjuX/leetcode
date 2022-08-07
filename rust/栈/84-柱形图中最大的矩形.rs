use std::cmp::max;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        if heights.len() == 0 {
            return 0
        }
        if heights.len() == 0 {
            return heights[0]
        }
        let len = heights.len() + 2;
        let mut stack = Vec::new();
        let mut new_heights = vec![0; len];
        for i in 0..heights.len() {
            new_heights[i + 1] = heights[i];
        }
        new_heights[0] = 0;
        new_heights[len - 1] = 0;
        let mut res  = 0;

        stack.push(new_heights[0]);
        for i in 1..len {
            while new_heights[i] < new_heights[stack[stack.len() - 1] as usize] {
                let h = new_heights[stack[stack.len() - 1] as usize];
                stack.pop();
                let w = i as i32 - stack[stack.len() - 1] - 1;
                res = max(res, w * h);
            } 
            stack.push(i as i32);
        }
        res
    }
}
