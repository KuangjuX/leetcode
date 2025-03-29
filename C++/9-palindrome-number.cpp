class Solution {
public:
    bool isPalindrome(int x) {
        std::vector<int> v;
        if(x < 0){
            return false;
        }
        while(x > 0){
            int num = x % 10;
            v.push_back(num);
            x /= 10;
        }
        int len = v.size();
        int front = 0, back = len - 1;
        while(front < back){
            if(v[front++] != v[back--])return false;
        }
        return true;
    }
};
