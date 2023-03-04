#include <vector>
#include <queue>
using namespace std;
class Solution {
public:
    struct compare {
        bool operator()(vector<int> a, vector<int> b) {
            return a[0] * a[0] + a[1] * a[1] > b[0] * b[0] + b[1] * b[1]; 
        }
    };

    vector<vector<int>> kClosest(vector<vector<int>>& points, int k) {
        priority_queue<vector<int>, vector<vector<int>>, compare> pq;
        vector<vector<int>> ans;
        for(auto &item: points) {
            pq.push(item);
        }
        for(int i = 0; i < k; i++) {
            ans.push_back(pq.top());
            pq.pop();
        }
        return ans;
    }
};