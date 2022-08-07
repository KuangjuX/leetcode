// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack1 = vec![];
        let mut stack2 = vec![];
        let mut stack = vec![];
        let mut node1 = l1.as_ref();
        let mut node2 = l2.as_ref();
        let mut root = Box::new(ListNode::new(0));
        let mut cur = &mut root;
        loop {
           match(node1, node2) {
               (Some(n1), Some(n2)) => {
                    stack1.push(n1.val);
                    stack2.push(n2.val);
                    node1 = n1.next.as_ref();
                    node2 = n2.next.as_ref();
               },

               (Some(n1), None) => {
                    stack1.push(n1.val);
                    node1 = n1.next.as_ref();
               },

               (None, Some(n2)) => {
                    stack2.push(n2.val);
                    node2 = n2.next.as_ref();
               },

               (None, None) => {
                    break;
               }
           }

        }

        let mut flag = 0;
        loop {
            match (stack1.pop(), stack2.pop()) {
                (Some(val1), Some(val2)) => {
                    stack.push((val1 + val2 + flag) % 10);
                    if val1 + val2 + flag > 9 {
                        flag = 1;
                    }else {
                        flag = 0;
                    }
                },

                (Some(val1), None) => {
                    stack.push((val1 + flag) % 10);
                    if val1 + flag > 9 {
                        flag = 1;
                    }else {
                        flag = 0;
                    }
                },

                (None, Some(val2)) => {
                    stack.push((val2 + flag) % 10);
                    if val2 + flag > 9 {
                        flag = 1;
                    }else {
                        flag = 0;
                    }
                },

                (None, None) => {
                    if flag == 1 {
                        stack.push(flag);
                    }
                    break;
                }
            }
        }

        loop {
            if let Some(val) = stack.pop() {
                let node = Box::new(ListNode::new(val));
                cur.next = Some(node);
                cur = cur.next.as_mut().unwrap();
            }else {
                break;
            }
        }
        root.next
    }
}