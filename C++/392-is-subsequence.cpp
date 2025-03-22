class Solution {
public:
    bool isSubsequence(string s, string t) {
        int x = 0, y = 0;
        char cx = s[x], cy = t[y];
        while(x < s.size() && y < t.size()){
            if(cx == cy){
                cx = s[++x];
                cy = t[++y];
            }else{
                cy = t[++y];
            }
        }
        return x == s.size();
    }
};
