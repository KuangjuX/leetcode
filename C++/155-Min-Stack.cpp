#include <stack>
using namespace std;

// @lc code=start
class MinStack {
    stack<int> s;
    stack<int> min_stack;
public:
    MinStack() {}
    
    void push(int val) {
        if(s.size() == 0){
            s.push(val);
            min_stack.push(val);
        }else{
            int min = std::min(min_stack.top(), val);
            s.push(val);
            min_stack.push(min);
        }
    }
    
    void pop() {
        s.pop();
        min_stack.pop();
    }
    
    int top() {
        return s.top();
    }
    
    int getMin() {
        return min_stack.top();
    }
};