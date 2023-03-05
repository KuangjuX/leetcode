#include <vector>
using namespace std;
class Solution {
public:
    vector<int> searchRange(vector<int>& nums, int target) {
        if(nums.size() == 0){
            return {-1, -1};
        }
        int low = 0;
        int high = nums.size() - 1;
        int mid = (low + high) / 2;
        int num = 0;
        vector<int> ans = {0, 0};
        while(1){
            num = nums[mid];
            if(low > high){
                ans[0] = -1;
                ans[1] = -1;
                break;
            }else{
                if(num == target){ break; }
                else if(num > target){ high = mid - 1; }
                else{ low = mid + 1; }
                mid = (low + high) / 2;
            }
        }
        if(num == target) {
            for(int i = mid; i < nums.size(); i++){
                if(nums[i] > num){ 
                    ans[1] = i - 1; 
                    break;
                }else if(i == nums.size() - 1){
                    ans[1] = i;
                }
            }
            for(int i = mid; i >= 0; i--){
                if(nums[i] < num){ 
                    ans[0] = i + 1;
                    break;
                }else if(i == 0){
                    ans[0] = 0;
                }
            }
        }
        return ans;
    }
};