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
type OptNode = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut r = 0;

        Self::search(root.clone(), &mut k, &mut r);

        r
    }

    pub fn search(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32, r: &mut i32) {
        if let Some(node) = root {
            Self::search(node.borrow().left.clone(), k, r);

            if *k == 1 {
                *r = node.borrow().val;
                *k -= 1;
                return;
            }
            *k -= 1;

            Self::search(node.borrow().right.clone(), k, r);
        }
    }
}

#[test]
fn test_fn() {
    // println!("{}", 5 / 2);
    // assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    // assert_eq!(get_row(0), vec![1]);
    // assert_eq!(get_row(1), vec![1, 1]);
}
