use std::cmp;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        // 字符的位置信息，记录字符最后一次出现的位置
        let mut pos: Vec<i32> = vec![-1;256];
        let mut max = 0;
        let mut slow: usize = 0;
        let mut fast: usize = 0;
        while fast < s.len() {
            if pos[s[fast] as usize] < slow as i32 {
                pos[s[fast] as usize] = fast as i32;
                fast += 1;
                max = cmp::max(max, fast - slow);
            }else {
                // pos[s[fast] as usize] = fast as i32;
                if pos[s[fast] as usize] == fast as i32 {
                    fast += 1;
                    max = cmp::max(max, fast - slow);
                }else {
                    pos[s[fast] as usize] = fast as i32;
                    slow += 1;
                    fast = slow;
                }
            }
        }
        max = cmp::max(max, fast - slow);
        max as i32
    }
}