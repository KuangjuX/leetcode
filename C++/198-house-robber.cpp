class Solution {
public:
    int rob(vector<int>& nums) {
        int n = nums.size();
        int records[n];
        if(n == 1)return nums[0];
        if(n == 2)return std::max(nums[0], nums[1]);
        records[0] = nums[0];
        records[1] = std::max(nums[0], nums[1]);
        for(int i = 2; i < n; ++i){
            records[i] = std::max(records[i - 2] + nums[i], records[i - 1]);
        }
        return records[n - 1];
    }
};
