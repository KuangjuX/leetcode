class Solution {
public:
    void merge(vector<int>& nums1, int m, vector<int>& nums2, int n) {
        int p1 = m - 1;
        int p2 = n - 1;
        int tail = m + n - 1;
        int cur = 0;
        for(int i = 0; i < m + n; ++i) {
            if(p1 == -1){
                cur = nums2[p2--];
            }else if(p2 == -1){
                cur = nums1[p1--];
            }else {
                if(nums1[p1] > nums2[p2]) {
                    cur = nums1[p1--];
                }else {
                    cur = nums2[p2--];
                }
            }
            nums1[tail--] = cur;
        }
    }
};
