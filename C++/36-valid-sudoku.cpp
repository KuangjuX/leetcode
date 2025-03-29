class Solution {
public:
    bool isValidSudoku(vector<vector<char>>& board) {
        for(int i = 0; i < 9; ++i){
            std::vector<bool> flag0(10, false);
            std::vector<bool> flag1(10, false);
            for(int j = 0; j < 9; ++j){
                // 检查第 i 行
                if(board[i][j] != '.'){
                    int num = board[i][j] - '0';
                    if(flag0[num])return false;
                    else flag0[num] = true;
                }
                // 检查第 i 列
                if(board[j][i] != '.'){
                    int num = board[j][i] - '0';
                    if(flag1[num])return false;
                    else flag1[num] = true;
                }
            }
        }

        for(int i = 0; i < 3; ++i){
            for(int j = 0; j < 3; ++j){
                std::vector<bool> flag(10, false);
                for(int m = 0; m < 3; ++m){
                    for(int n = 0; n < 3; ++n){
                        int row = i * 3 + m; 
                        int col = j * 3 + n;
                        if(board[row][col] != '.'){
                            int num = board[row][col] - '0';
                            if(flag[num])return false;
                            else flag[num] = true;
                        }
                    }
                }
            }
        }
        return true;
    }
};
