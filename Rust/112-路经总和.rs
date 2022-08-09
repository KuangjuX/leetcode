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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, mut target_sum: i32) -> bool {
        if let Some(node) = root {
            // let tree_node = node.borrow();
            if node.borrow().left == None && node.borrow().right == None {
                if node.borrow().val == target_sum {
                    return true
                }
            }else if node.borrow().left == None {
                target_sum -= node.borrow().val;
                let right_node = node.borrow_mut().right.take();
                return Self::has_path_sum(right_node, target_sum)
            }else if node.borrow().right == None {
                target_sum -= node.borrow().val;
                let left_node = node.borrow_mut().left.take();
                return Self::has_path_sum(left_node, target_sum)
            }else {
                target_sum -= node.borrow().val;
                let left_node = node.borrow_mut().left.take();
                let right_node = node.borrow_mut().right.take();
                return Self::has_path_sum(left_node, target_sum) | Self::has_path_sum(right_node, target_sum)
            }
        }
        false
    }
}