#include <string>
using namespace std;
class Solution {
public:
    int longestPalindrome(string s) {
        int table[255] = {0};
        int single = 0;
        int ans = 0;
        for(auto &c: s){
            if(table[c] == 0){ 
                table[c] += 1;
                single += 1;
            }
            else if(table[c] == 1){
                table[c] = 0;
                ans += 2;
                single -= 1;
            }
        }
        if(single > 0)ans += 1;
        return ans;
    }
};