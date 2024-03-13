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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        fn is_symmetric(
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if p.is_none() || q.is_none() {
                return p.is_none() && q.is_none();
            }

            let p = p.as_ref().unwrap().borrow();
            let q = q.as_ref().unwrap().borrow();

            p.val == q.val
                && is_symmetric(p.left.clone(), q.right.clone())
                && is_symmetric(p.right.clone(), q.left.clone())
        }

        let rb = root.as_ref().unwrap().borrow();
        is_symmetric(rb.left.clone(), rb.right.clone())
    }
}

#[test]
fn test_fn() {
    // println!("{}", 5 / 2);
    // assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    // assert_eq!(get_row(0), vec![1]);
    // assert_eq!(get_row(1), vec![1, 1]);
}
