class Solution {
public:
    int n, m;

    void dfs(vector<vector<char>>& board, int x, int y){
        if(x < 0 || x >= n || y < 0 || y >= m || board[x][y] != 'O')return;

        board[x][y] = 'A';
        dfs(board, x+1, y);
        dfs(board, x-1, y);
        dfs(board, x, y+1);
        dfs(board, x, y-1);
    }


    void solve(vector<vector<char>>& board) {
        n = board.size();
        m = board[0].size();
        if(n == 0){
            return;
        }

        for(int x = 0; x < n; ++x){
            dfs(board, x, 0);
            dfs(board, x, m - 1);
        }

        for(int y = 0; y < m; ++y){
            dfs(board, 0, y);
            dfs(board, n - 1, y);
        }

        for(int x = 0; x < n; ++x){
            for(int y = 0; y < m; ++y){
                if(board[x][y] == 'A'){
                    board[x][y] = 'O';
                }else if(board[x][y] == 'O'){
                    board[x][y] = 'X';
                }
            }
        }
    }
};
