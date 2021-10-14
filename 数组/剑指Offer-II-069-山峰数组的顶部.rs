impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut res = 0;
        for (i, ele) in arr.iter().enumerate() {
            if *ele > max {
                max = *ele;
                res = i;
            }
        }
        res as i32
    }
}