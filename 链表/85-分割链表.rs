impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut small_stack = vec![];
        let mut big_stack = vec![];
        let mut cur = head.as_ref();
        let mut root = Box::new(ListNode::new(0));
        let mut n = &mut root;
        while let Some(node) = cur {
            if node.val < x {
                small_stack.push(node.val);
            }else {
                big_stack.push(node.val);
            }
            cur = node.next.as_ref();
        }
        for val in small_stack {
            n.next = Some(Box::new(ListNode::new(val)));
            n = n.next.as_mut().unwrap();
        }
        for val in big_stack {
            n.next = Some(Box::new(ListNode::new(val)));
            n = n.next.as_mut().unwrap();
        }
        root.next
    }
}