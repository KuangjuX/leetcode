impl Solution {
    pub fn backtrace(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, ans: &mut Vec<i32>, visted: &mut Vec<bool>) {
        let len = nums.len();
        if ans.len() == len {
            res.push(ans.clone());
        }
        for i in 0..len {
            if visted[i] {
                continue;
            }else {
                ans.push(nums[i]);
                visted[i] = true;
                Self::backtrace(nums, res, ans, visted);
                ans.pop();
                visted[i] = false;
            }
        }
    }
    
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let mut ans = vec![];
        let mut visted = vec![false; nums.len()];
        Self::backtrace(&nums, &mut res, &mut ans, &mut visted);
        res
    }
}