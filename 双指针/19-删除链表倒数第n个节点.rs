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
        let mut slow = head.as_ref();
        let mut root = ListNode::new(0);
        let mut cur = &mut root;
        for _ in 0..n {
            if let Some(node) = fast {
                fast = node.next.as_ref();
            } else {
                return None;
            }
        }

        loop {
            if let Some(node) = fast {
                fast = node.next.as_ref();
                let val = slow.unwrap().val;
                slow = slow.unwrap().next.as_ref();
                cur.next = Some(Box::new(ListNode::new(val)));
                cur = cur.next.as_mut().unwrap();
            } else {
                // let cur = slow.as_mut();
                break;
            }
        }

        cur.next = slow.unwrap().next.clone();
        // root.next.take()
        root.next
    }
}