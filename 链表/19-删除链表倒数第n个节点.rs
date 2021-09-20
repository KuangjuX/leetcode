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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut fast = head.as_ref();
        let mut len = 0;
        loop {
            if let Some(node) = fast {
                fast = node.next.as_ref();
                len += 1;
            } else {
                break;
            }
        }
        drop(fast);

        if len == 1 {
            return None;
        }

        let mut slow = head.as_mut();
        if n == len {
          if let Some(node) = slow {
            return node.next.take()
          }else {
            return None
          }
        }
        for _ in 0..(len - 1 - n) {
            if let Some(node) = slow {
                slow = node.next.as_mut();
            } else {
                return None;
            }
        }

        if let Some(node) = slow {
            if let Some(next) = node.next.as_mut() {
                node.next = next.next.take();
            }
        }
        head
    }
}