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
    pub fn get_target(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut target_sum: i32,
        mut path: Vec<i32>,
        target: &mut Vec<Vec<i32>>,
    ) {
        if let Some(node) = root {
            if node.borrow().left == None && node.borrow().right == None {
                if target_sum == node.borrow().val {
                    path.push(node.borrow().val);
                    target.push(path);
                    return
                }
            }else if node.borrow().left == None {
                target_sum -= node.borrow().val;
                path.push(node.borrow().val);
                let right_node = node.borrow_mut().right.take();
                Self::get_target(right_node, target_sum, path, target)
            }else if node.borrow().right == None {
                target_sum -= node.borrow().val;
                path.push(node.borrow().val);
                let left_node = node.borrow_mut().left.take();
                Self::get_target(left_node, target_sum, path, target)
            }else {
                target_sum -= node.borrow().val;
                path.push(node.borrow().val);
                let left_node = node.borrow_mut().left.take();
                let right_node = node.borrow_mut().right.take();
                Self::get_target(left_node, target_sum, path.clone(), target);
                Self::get_target(right_node, target_sum, path.clone(), target)
            }
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
      let mut target = vec![];
      let path = vec![];
      Self::get_target(root, target_sum, path, &mut target);
      target
    }
}