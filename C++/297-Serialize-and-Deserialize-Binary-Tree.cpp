/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
#include <string>
#include <queue>
#include <sstream>
#include <iostream>
using namespace std;
class Codec {
public:

    // Examples: [1, 2, 3, null, null, 4, 5]
    // 
    // Encodes a tree to a single string.
    string serialize(TreeNode* root) {
        if(root == nullptr)return "";
        queue<TreeNode*> q;
        string s;
        q.push(root);
        while(!q.empty()) {
            TreeNode* node = q.front();
            q.pop();
            if(node != nullptr) {
                s += to_string(node->val);
                s += ',';
                q.push(node->left);
                q.push(node->right);
            }else{
                s += "#,";
            }
        }
        return s;
    }

    // Decodes your encoded data to tree.
    TreeNode* deserialize(string data) {
        if (data.size() == 0)return nullptr;
        stringstream ss(data);
        string item;
        queue<string> elements;
        while (getline(ss, item, ',')) {
            if (!item.empty()) {
                elements.push(item);
            }
        }
        if(elements.front() == "#")return nullptr;
        int value = stoi(elements.front());
        elements.pop();
        TreeNode* root = new TreeNode(value);
        queue<TreeNode*> nodes;
        nodes.push(root);
        while(!elements.empty()) {
            auto node = nodes.front();
            nodes.pop();
            auto left = elements.front();
            elements.pop();
            auto right = elements.front();
            elements.pop();
            if(left != "#"){
                node->left = new TreeNode(stoi(left));
                nodes.push(node->left);
            }
            if(right != "#"){
                node->right = new TreeNode(stoi(right));
                nodes.push(node->right);
            }
        }
        return root;
    }
};