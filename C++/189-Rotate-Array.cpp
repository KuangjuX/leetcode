/*
 * @lc app=leetcode.cn id=189 lang=cpp
 *
 * [189] 轮转数组
 */

// @lc code=start
#include <vector>
using namespace std;
class Solution {
public:
    // reverse array by start and end index.
    // Time complexity is O(end - start), Space complexity is O(1)
    void reverse(vector<int>& nums, int start, int end) {
        int size = (end - start) / 2;
        for(int i = start; i <= start + size; i++) {
            int temp = nums[i];
            nums[i] = nums[end + start - i];
            nums[end + start - i] = temp;
        }
    }

    /**
     * @brief 
     * 
     * @param nums 
     * @param k 
     * 
     * Solutions: 3 reverse
     * examples: [1, 2, 3, 4, 5, 6, 7]
     * reverse(0, size - k - 1): [4, 3, 2, 1, 5, 6, 7]
     * reverse(size - k, size - 1): [4, 3, 2, 1, 7, 6, 5]
     * reverse(0, size - 1): [5, 6, 7, 1, 2, 3, 4]
     * Time complexity: O(n), Space complexity: O(1)
     * 2010 408
     */
    void rotate(vector<int>& nums, int k) {
        int size = nums.size();
        // k mod size to maintain k in [0, size - 1]
        k = k % size;
        if(k == 0){
            return;
        }
        reverse(nums, 0, size - k - 1);
        reverse(nums, size - k, size - 1);
        reverse(nums, 0, size - 1);
    }
};
// @lc code=end

