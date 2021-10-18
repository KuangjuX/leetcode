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
use std::cmp::{max, min};
impl Solution {
    // 是否为二叉数，最大值，最小值
    pub fn dfs(node: Rc<RefCell<TreeNode>>) -> (bool, i32, i32) {
        let mut node_borrow = node.borrow_mut();
        let val = node_borrow.val;
        match (node_borrow.left.take(), node_borrow.right.take()) {
            (Some(left), Some(right)) => {
                let (left_res, left_max, left_min) = Self::dfs(left);
                let (right_res, right_max, right_min) = Self::dfs(right);
                if val > left_max && val < right_min && left_res && right_res {
                    return (true, right_max, left_min)
                }else {
                    let max = max(val, right_max);
                    let min = min(val, left_min);
                    return (false, max, min) 
                }
            },

            (Some(left), None) => {
                let (left_res, left_max, left_min) = Self::dfs(left);
                if val > left_max && left_res{
                    return (true, val, left_min)
                }else {
                    let max = max(val, left_max);
                    let min = min(val, left_min);
                    return (false, max, min)
                }
            },

            (None, Some(right)) => {
                let (right_res, right_max, right_min) = Self::dfs(right);
                if val < right_min && right_res{
                    return (true, right_max, val)
                }else {
                    let max = max(val, right_max);
                    let min = min(val, right_min);
                    return (false, max, min)
                }
            },

            (None, None) => {
                return (true, val, val)
            }
        }
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let (ans, _, _) = Self::dfs(node);
            ans
        }else {
            false
        }
    }
}