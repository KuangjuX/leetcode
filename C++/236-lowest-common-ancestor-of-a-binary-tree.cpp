/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    bool dfs(TreeNode* root, TreeNode* p, TreeNode* q, TreeNode** ans){
        if(root == nullptr)return false;
        bool left = dfs(root->left, p, q, ans);
        bool right = dfs(root->right, p, q, ans);
        if(left && right && (*ans == nullptr)){
            *ans = root;
            return true;
        }
        if(root == p || root == q){
             if(left || right){
                *ans = root;
             }
             return true;
        }
        return (left || right);
    }

    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        TreeNode* ans = nullptr;

        dfs(root, p, q, &ans);

        return ans;
    }
};
