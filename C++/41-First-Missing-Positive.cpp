/*
 * @lc app=leetcode.cn id=41 lang=cpp
 *
 * [41] 缺失的第一个正数
 */

// @lc code=start
/**
 * @file 41-First-Missing-Positive.cpp
 * @author your name (you@domain.com)
 * @brief 
 * @version 0.1
 * @date 2022-08-11
 * 
 * @copyright Copyright (c) 2022
 * 
 * ref: (https://leetcode.cn/problems/first-missing-positive/solution/que-shi-de-di-yi-ge-zheng-shu-by-leetcode-solution/)
 * This question is the 2018 408 exam question, we use native array to implement
 * hash table to find first missing positive number. Its time complexity is O(n) and 
 * space complexity is O(1).
 * 
 */
#include <vector>
using namespace std;
class Solution {
public:
    int firstMissingPositive(vector<int>& nums) {
        int size = nums.size();
        // turn all negative numbers into postive numbers
        for(int i = 0; i < size; i++) {
            if(nums[i] <= 0) {
                nums[i] = size + 1;
            }
        }

        for(int i = 0; i < size; i++) {
            if(nums[i] > 0 && nums[i] <= size) {
                if(nums[nums[i] - 1] > 0) {
                    nums[nums[i] - 1] = -nums[nums[i] - 1];
                }
            }else if(nums[i] < 0 && nums[i] >= -size){
                int num = -nums[i];
                if(nums[num - 1] > 0) {
                    nums[num - 1] = -nums[num - 1];
                }
            }
        }

        for(int i = 0; i < size; i++) {
            if(nums[i] > 0 && i >= 0) {
                return i + 1;
            }
        }
        
        return size + 1;
    }
};
// @lc code=end

