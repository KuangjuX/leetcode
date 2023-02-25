#include <vector>
#include <algorithm>
using namespace std;
class Solution {
public:
    vector<vector<int>> merge(vector<vector<int>>& intervals) {
        if(intervals.size() == 0)return {};
        sort(intervals.begin(), intervals.end()); 
        vector<vector<int>> merged;
        merged.push_back(intervals[0]);
        for(int i = 1; i < intervals.size(); i++){
            int pos = merged.size() - 1;
            if(merged[pos][1] < intervals[i][0]) {
                merged.push_back(intervals[i]);
            }else{
                merged[pos][1] = max(merged[pos][1], intervals[i][1]);
            }
        }
        return merged;
    }
};