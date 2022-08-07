#include <vector>
#include <stack>
#include <stdio.h>
using namespace std;

class Solution {
public:
    vector<int> dailyTemperatures(vector<int>& temperatures) {
        size_t len = temperatures.size();
        stack<int> s;
        vector<int> ans(len, 0);
        for(size_t i = 0; i < len; i++) {
            while(!s.empty()) {
                if(temperatures[i] > temperatures[s.top()]) {
                    ans[s.top()] = i - s.top();
                    s.pop();
                }else{ break; }
            }
            s.push(i);
        }
        return ans;
    }
};

int main() {
    Solution solution = Solution();
    vector<int> temperatures = {89,62,70,58,47,47,46,76,100,70};
    auto ans = solution.dailyTemperatures(temperatures);
    for(size_t i = 0; i < ans.size(); i++) {
        printf("%d ", ans[i]);
    }
    printf("\n");
    
}