mod solutions_1_30;
struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_valid_bst(
            node: &Option<Rc<RefCell<TreeNode>>>,
            min: Option<i32>,
            max: Option<i32>,
        ) -> bool {
            if node.is_none() {
                return true;
            }

            let val = node.as_ref().unwrap().borrow().val;
            if let Some(min) = min {
                if val <= min {
                    return false;
                }
            }
            if let Some(max) = max {
                if val >= max {
                    return false;
                }
            }

            is_valid_bst(&node.as_ref().unwrap().borrow().left, min, Some(val))
                && is_valid_bst(&node.as_ref().unwrap().borrow().right, Some(val), max)
        }

        is_valid_bst(&root, None, None)
    }
}

#[test]
fn test_fn() {
    // println!("{}", 5 / 2);
    // assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    // assert_eq!(get_row(0), vec![1]);
    // assert_eq!(get_row(1), vec![1, 1]);
}
