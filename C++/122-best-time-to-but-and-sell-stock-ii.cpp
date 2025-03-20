class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int n = prices.size();
        int dp0 = 0, dp1 = -prices[0];
        for(int i = 0; i < n; ++i){
            dp0 = std::max(dp0, dp1 + prices[i]);
            dp1 = std::max(dp1, dp0 - prices[i]);
        }
        return dp0;
    }
};
