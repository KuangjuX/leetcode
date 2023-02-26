#include <vector>
#include <iterator>
using namespace std;
class Solution {
public:
    int removeElement(vector<int>& nums, int val) {
        int num = 0;
        auto iter = nums.begin();
        int index = 0;
        while(index < nums.size()) {
            if(nums[index] == val) {
                nums.erase(nums.begin() + index);
            }else {
                index++;
                num++;
            }
        }
        return num;
    }
};