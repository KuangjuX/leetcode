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
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut root = ListNode::new(0);
        let mut pos = &mut root;
        let mut stack = Vec::new();
        let mut count = k;
        if head == None {
            return None
        }
        while let Some(mut node) = head.take() {
            head = node.next.take();
            if count > 0 {
                stack.push(node);
                count -= 1;
            }else if count == 0 {
                for _ in 0..k as usize {
                    let node = stack.pop();
                    pos.next = node;
                    pos = pos.next.as_mut().unwrap();
                }
                count = k;
                stack.push(node);
                count -= 1;
            }
        }
        if count == 0 {
            while stack.len() > 0 {
                let node = stack.pop();
                pos.next = node;
                pos = pos.next.as_mut().unwrap();
            }
        }else {
            for node in stack {
                pos.next = Some(node);
                pos = pos.next.as_mut().unwrap();
            }
        }
        root.next
    }
}