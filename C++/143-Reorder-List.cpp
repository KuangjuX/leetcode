/*
 * @lc app=leetcode.cn id=143 lang=cpp
 *
 * [143] 重排链表
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

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};


/**
 * @brief This question is the 2019 408 exam question,
 * you can find this question in P42 in WangDao. 
 * The time complexity is O(n) and space complexity is O(1)
 * 
 */
class Solution {
public:
    void reorderList(ListNode* head) {
        // Get middle node
        ListNode *p, *q;
        p = head;
        q = head;
        while(p->next != nullptr) {
            p = p->next;
            q = q->next;
            if(p->next != nullptr) {
                p = p->next;
            }
        }
        // Now node p is tail node and q is middle node
        // Reverse List between [middle, tail]
        ListNode* prev = nullptr;
        ListNode* cur = q;
        ListNode* next = q->next;
        while(next != nullptr) {
            ListNode* x = next->next;
            cur->next = prev;
            next->next = cur;
            prev = cur;
            cur = next;
            next = x;
        }

        // Reorder List from head node
        // Now previous tail head is middle node
        cur = p;
        ListNode* node = head;
        while(node->next != p && cur->next != nullptr) {
            next = node->next;
            node->next = cur;
            node = next;
            next = cur->next;
            cur->next = node;
            cur = next;
        }
    }
};
// @lc code=end

