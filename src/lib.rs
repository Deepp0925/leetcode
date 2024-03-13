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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<usize> {
            let Some(root) = root else { return Some(0) };
            let r = root.borrow();

            let lheight = height(&r.left)?;
            let rheight = height(&r.right)?;

            if lheight == rheight + 1 || lheight == rheight {
                Some(lheight + 1)
            } else if lheight + 1 == rheight {
                Some(rheight + 1)
            } else {
                None
            }
        }

        height(&root).is_some()
    }
}

#[test]
fn test_fn() {
    // println!("{}", 5 / 2);
    // assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    // assert_eq!(get_row(0), vec![1]);
    // assert_eq!(get_row(1), vec![1, 1]);
}
