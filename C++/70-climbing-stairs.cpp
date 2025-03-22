class Solution {
public:
    int climbStairs(int n) {
        int records [n + 1];
        if(n == 1)return 1;
        records[0] = 1;
        records[1] = 1;
        for(int i = 2; i < n + 1; ++i){
            records[i] = records[i - 1] + records[i - 2];
        }
        return records[n];
    }
};
