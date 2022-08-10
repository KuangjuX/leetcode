/*
 * @lc app=leetcode.cn id=61 lang=cpp
 *
 * [61] 旋转链表
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

// This question is the 408 exam question in 2010,
// which is a rotate left
// The time complexity is O(n) and space complexity is O(1)
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    ListNode* rotateRight(ListNode* head, int k) {
        if(head == nullptr || head->next == nullptr) {
            return head;
        }
        int len = 0;
        ListNode* node = head;
        ListNode* tail = head;
        while(tail->next != nullptr) {
            len++;
            tail = tail->next;
        }
        len++;
        k = k % len;
        if(k == 0){
            return head;
        }
        for(int i = 0; i < len - k - 1; i++) {
            node = node->next;
        }
        ListNode* cur = node;
        node = node->next;
        cur->next = nullptr;
        tail->next = head;
        return node;
    }
};
// @lc code=end

