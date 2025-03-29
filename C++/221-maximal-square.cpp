class Solution {
public:
    int maximalSquare(vector<vector<char>>& matrix) {
        int m = matrix.size();
        int n = matrix[0].size();
        vector<vector<int>> dp(m, vector(n, 0));
        int max = 0;

        for(int i = 0; i < m; ++i){
            if(matrix[i][0] == '1'){
                dp[i][0] = 1;
                max = 1;
            }
        }
        for(int j = 0; j < n; ++j){
            if(matrix[0][j] == '1'){
                dp[0][j] = 1;
                max = 1;
            }
        }

        for(int i = 1; i < m; ++i){
            for(int j = 1; j < n; ++j){
                if(matrix[i][j] == '1'){
                    dp[i][j] = std::min(dp[i-1][j], std::min(dp[i-1][j-1], dp[i][j-1])) + 1;
                    max = std::max(max, dp[i][j]);
                }else{
                    dp[i][j] = 0;
                }
            }
        }
        return max * max;
    }
};
