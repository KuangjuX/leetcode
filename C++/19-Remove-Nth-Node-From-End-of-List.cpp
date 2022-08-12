/*
 * @lc app=leetcode.cn id=19 lang=cpp
 *
 * [19] 删除链表的倒数第 N 个结点
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


class Solution {
public:
    /**
     * @brief 
     * 
     * @param head 
     * @param n 
     * @return ListNode* 
     * 2009 408 exam question
     * double pointer: Time complexity: O(n), Space complexity: O(1)
     */
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        if(head == nullptr || head->next == nullptr){
            return nullptr;
        }
        struct ListNode* fast = head;
        struct ListNode* slow = head;
        for(int i = 0; i < n; i++) {
            fast = fast->next;
        }
        if(fast == nullptr) {
            return head->next;
        }
        while(fast->next != nullptr) {
            fast = fast->next;
            slow = slow->next;
        }
        slow->next = slow->next->next;
        return head;
    }
};
// @lc code=end

