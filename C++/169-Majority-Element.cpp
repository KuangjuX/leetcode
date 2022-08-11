/*
 * @lc app=leetcode.cn id=169 lang=cpp
 *
 * [169] 多数元素
 */

// @lc code=start

/**
 * @file 169-Majority-Element.cpp
 * @author your name (you@domain.com)
 * @brief 
 * @version 0.1
 * @date 2022-08-11
 * 
 * @copyright Copyright (c) 2022
 * This question is the 2018 408 exam question,
 * This solution use hash table to store the frequency of all number
 * time complexity is O(n), space complexity is O(n).
 * Other solution ref: (https://leetcode.cn/problems/majority-element/solution/duo-shu-yuan-su-by-leetcode-solution/)
 */
#include <unordered_map>
#include <vector>
#include <stdio.h>
using namespace std;
class Solution {
public:
    int majorityElement(vector<int>& nums) {
        unordered_map<int, int> map;
        int size = nums.size();
        for(int i = 0; i < size; i++) {
            if(map.find(nums[i]) != map.end()) {
                auto element = map.find(nums[i]);
                element->second += 1;
            }else{
                map.insert(make_pair(nums[i], 1));
            }
        }

        for(auto& x: map) {
            if(x.second > size / 2) {
                return x.first;
            }
        }
        return -1;
    }
};
// @lc code=end

