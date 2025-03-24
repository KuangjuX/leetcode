class Solution {
public:
    int minPathSum(vector<vector<int>>& grid) {
       int m = grid.size();
       int n = grid[0].size();
       std::vector<std::vector<int>> path(m, std::vector<int>(n , 0));

       path[0][0] = grid[0][0];

       for(int i = 1; i < m; ++i){
            path[i][0] = path[i-1][0] + grid[i][0];
       } 

       for(int j = 1; j < n; ++j){
            path[0][j] = path[0][j-1] + grid[0][j];
       }

       for(int i = 1; i < m; ++i){
            for(int j = 1; j < n; ++j){
                path[i][j] = std::min(path[i-1][j], path[i][j-1]) + grid[i][j];
            }
       }

       return path[m-1][n-1];
    }
};
