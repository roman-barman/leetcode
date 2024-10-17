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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        Self::is_valid_node(root, i64::MIN, i64::MAX)
    }

    pub fn is_valid_node(tree_node: Option<Rc<RefCell<TreeNode>>>, left: i64, right: i64) -> bool {
        if tree_node.is_none() {
            return true;
        }

        let tree_node = tree_node.unwrap();
        let tree_node= RefCell::borrow_mut(&tree_node);
        let val = tree_node.val as i64;
        let result = val > left && val < right;

        result && Self::is_valid_node(tree_node.left.clone(), left, val) && Self::is_valid_node(tree_node.right.clone(), val, right)
    }
}