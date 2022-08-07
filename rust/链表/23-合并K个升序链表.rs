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
    pub fn merge_2_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = Box::new(ListNode::new(0));
        let mut p = root.as_mut();
        let mut p1 = l1.as_ref();
        let mut p2 = l2.as_ref();
        loop {
            if p1 == None && p2 == None {
                return root.next
            }else if p1 == None {
                while p2 != None {
                    p.next = Some(Box::new(ListNode::new(p2.unwrap().val)));
                    p = p.next.as_mut().unwrap();
                    p2 = p2.unwrap().next.as_ref();
                }
                return root.next
            }else if p2 == None {
                while p1 != None {
                    p.next = Some(Box::new(ListNode::new(p1.unwrap().val)));
                    p = p.next.as_mut().unwrap();
                    p1 = p1.unwrap().next.as_ref();
                }
                return root.next
            }else {
                let v1 = p1.unwrap().val;
                let v2 = p2.unwrap().val;
                if v1 < v2 {
                    p.next = Some(Box::new(ListNode::new(v1)));
                    p1 = p1.unwrap().next.as_ref();
                }else {
                    p.next = Some(Box::new(ListNode::new(v2)));
                    p2 = p2.unwrap().next.as_ref();
                }
            }
            p = p.next.as_mut().unwrap();
        }
    }

    pub fn split(mut l1: Vec<Option<Box<ListNode>>>, mut l2: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if l1.len() == 1 && l2.len() == 1 {
            return Self::merge_2_lists(l1[0].clone(), l2[0].clone())
        }else if l1.len() == 1 && l2.len() == 2 {
            return Self::merge_2_lists(l1[0].clone(), Self::merge_2_lists(l2[0].clone(), l2[1].clone()))
        }else if l1.len() == 2 && l2.len() == 1 {
            return Self::merge_2_lists(Self::merge_2_lists(l1[0].clone(), l1[1].clone()), l2[0].clone())
        }else {
            let l12 = l1.split_off(l1.len()/2);
            let l22 = l2.split_off(l2.len()/2);
            return Self::merge_2_lists(Self::split(l1, l12), Self::split(l2, l22))
        }
    }

    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None
        }else if lists.len() == 1 {
            return lists[0].clone()
        }else {
            let l = lists.split_off(lists.len()/2);
            return Self::split(lists, l);
        }
    }
}