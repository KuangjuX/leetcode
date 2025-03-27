class Solution {
public:
    void dfs(int high, int low, vector<vector<int>>& matrix){
        if(high >= low)return;
        for(int i = high; i <= low - 1; ++i){
            // 旋转四个角
            int temp0 = matrix[i][low];
            // 旋转第一个
            matrix[i][low] = matrix[high][i];
            // 旋转第二个
            int temp1 = matrix[low][low + high - i];
            matrix[low][low + high - i] = temp0;
            // 旋转第三个
            int temp2 = matrix[low + high - i][high];
            matrix[low + high - i][high] = temp1;
            // 旋转第四个
            matrix[high][i] = temp2;
        }
        dfs(high + 1, low - 1, matrix);
    }

    void rotate(vector<vector<int>>& matrix) {
        int n = matrix.size();
        dfs(0, n-1, matrix);
    }
};
