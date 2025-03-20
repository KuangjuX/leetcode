class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int n = prices.size();
        int max_profit = 0, min_profit = 10001;

        for(int i = 0; i < n; ++i){
            max_profit = std::max(prices[i] - min_profit, max_profit);
            min_profit = std::min(min_profit, prices[i]);
        }

        return max_profit;
    }
};
