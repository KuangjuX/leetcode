impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![];
        dp.push(1);
        dp.push(2);
        for i in 2..19 {
            let mut sum = 0;
            for j in 0..=i {
                if j == 0 {
                    sum += dp[i - 1];
                }else if i == j {
                    sum += dp[i - 1];
                }
                else {
                    sum += dp[j - 1] * dp[i - j - 1];
                }
            }
            dp.push(sum);
        }
        dp[n-1]
    }
}
