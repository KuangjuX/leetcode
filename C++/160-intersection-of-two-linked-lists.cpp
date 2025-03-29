/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        ListNode *pA = headA, *pB = headB;
        bool transverA = false, transverB = false;
        while(pA != NULL && pB != NULL){
            if(pA != pB){
                if(pA->next != NULL)pA = pA->next;
                else if(!transverA){
                    pA = headB;
                    transverA = true;
                }else pA = NULL;

                if(pB->next != NULL)pB = pB->next;
                else if(!transverB){
                    pB = headA;
                    transverB = true;
                }else pB = NULL;
            }else{
                return pA;
            }
        }
        return pA;
    }
};
