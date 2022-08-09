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
    pub fn tree_depth(node: Rc<RefCell<TreeNode>>) -> i32 {
        if node.borrow().left == None && node.borrow().right == None {
            return 1
        }else if node.borrow().left == None {
            return 1 + Self::tree_depth(node.borrow_mut().right.take().unwrap())
        } else if node.borrow().right == None {
            return 1 + Self::tree_depth(node.borrow_mut().left.take().unwrap())
        }else {
            let left = node.borrow_mut().left.take().unwrap();
            let right = node.borrow_mut().right.take().unwrap();
            let left_child = Rc::clone(&left);
            let right_child = Rc::clone(&right);
            return max(
                Self::tree_depth(left_child), 
                Self::tree_depth(right_child)
            ) + 1
        }
    }
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                Self::tree_depth(node)
            },

            None => {
                0
            }
        }
    }
}
