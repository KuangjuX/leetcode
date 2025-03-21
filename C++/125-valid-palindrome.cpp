class Solution {
public:
    bool isPalindrome(string s) {
        int n = s.size();
        int x = 0, y = n - 1;
        int gap = 'a' - 'A';
        while(x < y){
            if(s[x] == ' ' || (!(s[x] >= 'A' && s[x] <= 'Z') && !(s[x] >= 'a' && s[x] <= 'z') && !(s[x] >= '0' && s[x] <= '9'))){
                std::cout << "x: " << s[x] << std::endl;
                ++x;
            }
            else if(s[y] == ' ' || (!(s[y] >= 'A' && s[y] <= 'Z') && !(s[y] >= 'a' && s[y] <= 'z') && !(s[y] >= '0' && s[y] <= '9'))){
                std::cout << "y: " << s[y] << std::endl;
                --y;
            }
            else{
                // std::cout << "x: " << s[x] << std::endl;
                // std::cout << "y: " << s[y] << std::endl;
                int m = std::abs(s[x] - s[y]);
                if(m == 0){
                    ++x;
                    --y;
                }else if(m == gap){
                    if(((s[x] >= 'A' && s[x] <= 'Z') || (s[x] >= 'a' && s[x] <= 'z')) && ((s[y] >= 'A' && s[y] <= 'Z') || (s[y] >= 'a' && s[x] <= 'y'))){
                        ++x;
                        --y;
                    }else{
                        return false;
                    }
                }
                else{
                    return false;
                }
            }
        }

        return true;
    }
};
