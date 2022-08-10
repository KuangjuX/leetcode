/*
 * @lc app=leetcode.cn id=876 lang=cpp
 *
 * [876] 链表的中间结点
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

/**
 * @brief The time complexity is O(n), space complexity is O(1)
 * 
 */
class Solution {
public:
    ListNode* middleNode(ListNode* head) {
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
        return q;
    }
};
// @lc code=end

