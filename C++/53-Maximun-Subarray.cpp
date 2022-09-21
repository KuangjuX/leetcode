#include <vector>
using namespace std;
class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int ans = nums[0];
        int sum = 0;
        for(auto& num: nums) {
            if(sum < 0) {
                sum = num;
            }else{
                sum += num;
            }
            ans = max(ans, sum);
        }
        return ans;
    }
};