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
        let mut node = head.unwrap();
        for i in 0..(n-2) {
            match node.next {
                Some(next) => {
                    node = &mut next;
                },

                None => {
                    return None
                }
            }
        }
        match &node.next {
            Some(remove_node) => {
                node.next = remove_node.next;
            },

            None => {
                return None
            }
        }
        head
    }
}