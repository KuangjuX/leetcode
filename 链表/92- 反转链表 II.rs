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
use std::slice;
impl Solution {
    pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let left = left as usize;
        let right = right as usize;
        let mut cur = head;
        let mut v = vec![];
        let mut root = Box::new(ListNode::new(0));
        let mut node = &mut root;

        loop {
            if let Some(node) = cur {
                cur = node.next;
                v.push(node.val);
            }else {
                break;
            }
        }
        let mid = unsafe {
            slice::from_raw_parts_mut(
                v.as_mut_ptr().offset((left - 1) as isize), 
                right - left + 1
            )
        };
        mid.reverse();

        for i in 0..v.len() {
            if i < left - 1 || i > right - 1 {
                let next = Box::new(ListNode::new(v[i]));
                node.next = Some(next);
                node = node.next.as_mut().unwrap();
            }else {
                let index = i - left + 1;
                let next = Box::new(ListNode::new(mid[index]));
                node.next = Some(next);
                node = node.next.as_mut().unwrap();
            }
        }

        root.next
    }
}