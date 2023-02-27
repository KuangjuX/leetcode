#include <queue>
using namespace std;

// @lc code=start
class MyStack {
public:
    queue<int> q1;
    queue<int> q2;
    int size;
    MyStack() {
        size = 0;
    }
    
    void push(int x) {
        size++;
        while(!q1.empty()){
            q2.push(q1.front());
            q1.pop(); 
        }
        q1.push(x);
        while(!q2.empty()){
            q1.push(q2.front());
            q2.pop();
        }
    }
    
    int pop() {
        int item = q1.front();
        q1.pop();
        size--;
        return item;
    }
    
    int top() {
        return q1.front();
    }
    
    bool empty() {
        return size == 0;
    }
};
