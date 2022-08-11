/*
 * @lc app=leetcode.cn id=4 lang=cpp
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
/**
 * @file 4-Median-of-Two-Sorted-Arrays.cpp
 * @author your name (you@domain.com)
 * @brief 
 * @version 0.1
 * @date 2022-08-11
 * 
 * @copyright Copyright (c) 2022
 * This problem is 2011 408 exam question
 * double pointer
 * Time Complexity: O(n), Space Complexity: O(1)
 */
#include <vector>
using namespace std;
class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
        int size_1 = nums1.size();
        int size_2 = nums2.size();
        int size = size_1 + size_2;
        // two pointer
        int i = 0, j = 0;
        // position counter
        int count = 0;
        double odd_medium_num = -1;
        // judge if size is odd number
        bool odd = (size % 2) != 0;
        int num1 = 0, num2 = 0;
        if(size_1 > 0) {
            num1 = nums1[0];
        }else{
            num1 = 1 << 30;
        }
        
        if(size_2 > 0){
            num2 = nums2[0];
        }else{
            num2 = 1 << 30;
        }
        while(i < size_1 || j < size_2) {
            count++;
            if(j >= size_2 || num1 < num2) {
                if(odd && count == size / 2 + 1){
                    return num1;
                }else if(count == size / 2 || count == size / 2 + 1 && !odd){
                    if(odd_medium_num < 0) {
                        odd_medium_num = (double)num1;
                    }else{
                        odd_medium_num += (double)num1;
                        return odd_medium_num / 2;
                    }
                }
                if(i >= size_1 - 1) {
                    num1 = 1 << 30;
                    ++i;
                }else{
                    num1 = nums1[++i];
                }
            }else if(i >= size_1 || num2 <= num1){
                if(odd && count == size / 2 + 1){
                    return num2;
                }else if(count == size / 2 || count == size / 2 + 1 && !odd){
                    if(odd_medium_num < 0) {
                        odd_medium_num = (double)num2;
                    }else{
                        odd_medium_num += (double)num2;
                        return odd_medium_num / 2;
                    }
                }
                if(j >= size_2 - 1) {
                    ++j;
                    num2 = 1 << 30;
                }else{
                    num2 = nums2[++j];
                }
            }
        }
        return 0;
    }
};
// @lc code=end

