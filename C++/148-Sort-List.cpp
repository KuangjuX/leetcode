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
#include <vector>
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};
class Solution {
public:
    void mergeSort(std::vector<int>& vec, int low, int high) {
        if(low == high) {
            return;
        }else if(low + 1 == high) {
            if(vec[low] > vec[high]) std::swap(vec[low], vec[high]);
            return;
        }else{
            int mid = (high + low) / 2;
            mergeSort(vec, low, mid);
            mergeSort(vec, mid + 1, high);
            merge(vec, low, mid, high);
            return;
        }
    }

    void merge(std::vector<int>& vec, int low, int mid, int high) {
        int i = low; 
        int j = mid + 1;
        std::vector<int> res;
        while(i <= mid || j <= high) {
            if(i > mid)res.push_back(vec[j++]);
            else if(j > high)res.push_back(vec[i++]);
            else{
                if(vec[i] < vec[j])res.push_back(vec[i++]);
                else res.push_back(vec[j++]);
            }
        }
        for(int i = 0; i < res.size(); i++) {
            vec[low + i] = res[i];
        }
    }

    ListNode* sortList(ListNode* head) {
        if(head == nullptr){return nullptr;}
        std::vector<int> vec;
        ListNode* node = head;
        while(node != nullptr) {
            vec.push_back(node->val);
            node = node->next;
        }
        if(vec.size() == 1){ return head; }
        int mid = (vec.size() - 1) / 2;
        mergeSort(vec, 0, mid);
        mergeSort(vec, mid + 1, vec.size() - 1);
        merge(vec, 0, mid, vec.size() - 1);
        node = head;
        for(auto &item: vec) {
            node->val = item;
            node = node->next;
        }
        return head;
    }
};