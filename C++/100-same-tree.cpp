/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    bool isSameTree(TreeNode* p, TreeNode* q) {
        if(p == nullptr && q == nullptr)return true;
        else if((p == nullptr && q != nullptr) || (p != nullptr && q == nullptr))return false;
        std::queue<TreeNode*> q0, q1;
        q0.push(p);
        q1.push(q);
        while(!q0.empty() || !q1.empty()){
            TreeNode* node0 = q0.front();
            TreeNode* node1 = q1.front();
            q0.pop();
            q1.pop();
            if(node0->val == node1->val){
                if(node0->left == nullptr && node1->left == nullptr){}
                else if((node0->left == nullptr && node1->left != nullptr) || (node0->left != nullptr && node1->left == nullptr))return false;
                else{
                    q0.push(node0->left);
                    q1.push(node1->left);
                }

                if(node0->right == nullptr && node1->right == nullptr){}
                else if((node0->right == nullptr && node1->right != nullptr) || (node0->right != nullptr && node1->right == nullptr))return false;
                else{
                    q0.push(node0->right);
                    q1.push(node1->right);
                }
            }else{
                return false;
            }
        }
        return true;
    }
};
