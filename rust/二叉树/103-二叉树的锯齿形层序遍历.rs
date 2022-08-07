// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn zigzag_level_order(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut res: Vec<Vec<i32>> = vec![];
        // true 表示从左向右， false表示从右向左
        let mut order = true;
        let node = root.take();
        if let Some(treenode) = node {
            queue.push(treenode)
        }else {
            return res
        }
        while !queue.is_empty() {
            if order {
                // 从左向右
                let len = queue.len();
                let mut tmp = vec![];
                let mut new_queue = vec![];
                for _ in 0..len {
                    let node = queue.pop().unwrap();
                    tmp.push(node.borrow().val);
                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    if let Some(left_node) = left {
                        new_queue.push(left_node);
                    }
                    if let Some(right_node) = right {
                        new_queue.push(right_node);
                    }
                }   
                queue = new_queue;
                res.push(tmp);
            }else {
                // 从右向左
                let len = queue.len();
                let mut tmp = vec![];
                let mut new_queue = vec![];
                for _ in 0..len {
                    let node = queue.pop().unwrap();
                    tmp.push(node.borrow().val);
                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    if let Some(right_node) = right {
                        new_queue.push(right_node);
                    }
                    if let Some(left_node) = left {
                        new_queue.push(left_node);
                    }
                }   
                res.push(tmp);
                queue = new_queue;
            }
            order = !order
        }
        res
    }
}