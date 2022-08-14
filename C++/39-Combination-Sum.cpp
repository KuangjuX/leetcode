/*
 * @lc app=leetcode.cn id=39 lang=cpp
 *
 * [39] 组合总和
 */

// @lc code=start
/**
 * @file 39-Combination-Sum.cpp
 * @author your name (you@domain.com)
 * @brief 
 * @version 0.1
 * @date 2022-08-14
 * 
 * @copyright Copyright (c) 2022
 * Backtrace O(n^2), O(n^2)
 * 
 */
#include <vector>
using namespace std;
class Solution {
public:
    void dfs(vector<int>& candidates, int target, int start, vector<int> x, vector<vector<int>>& ans){
        if(target == 0){
                ans.push_back(x);
                return;
        }else if(target < 0){
                return;
        }else{
            for(int i = start; i < candidates.size(); i++) {
                target -= candidates[i];
                x.push_back(candidates[i]);
                dfs(candidates, target, i, x, ans);
                target += candidates[i];
                x.pop_back();
                
            }
        }
    }

    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        vector<vector<int>> ans;
        vector<int> x;
        dfs(candidates, target, 0, x, ans);
        return ans;
    }
};

