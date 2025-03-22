class Solution {
public:
    int lengthOfLIS(vector<int>& nums) {
        int n = nums.size();
        std::vector<int> records(n, 1);
        int ans = 1;
        for(int i = 1; i < n; ++i){
            int num = nums[i];
            for(int j = i - 1; j >= 0; --j){
                if(nums[j] < num){
                    records[i] = std::max(records[j] + 1, records[i]);
                    ans = std::max(ans, records[i]);
                }            
            }
        }
        return ans;
    }
};
