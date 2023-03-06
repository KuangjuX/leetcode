#include <vector>
using namespace std;
class Solution {
public:
    int singleNonDuplicate(vector<int>& nums) {
        // 数组长度一定是奇数，根据偏向哪边进行选择
        int low = 0;
        int high = nums.size() - 1;
        int mid;
        while(low < high) {
            mid = (high - low) / 2 + low;
            if(mid == 0 || mid == nums.size() - 1)break;
            int num = nums[mid];
            int left = nums[mid - 1];
            int right = nums[mid + 1];
            if(mid % 2 == 0){
                if(num == right)low = mid + 1;
                else high = mid;
            }else{
                if(num == left)low = mid + 1;
                else high = mid;
            }
        }
        return nums[low];
    }
};