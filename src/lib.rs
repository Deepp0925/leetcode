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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = vec![];

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
            if node.is_none() {
                return;
            }

            let node = node.as_ref().unwrap().borrow();
            dfs(node.left.clone(), v);
            dfs(node.right.clone(), v);
            v.push(node.val);
        }

        dfs(root, &mut v);

        v
    }
}

#[test]
fn test_fn() {
    // println!("{}", 5 / 2);
    // assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    // assert_eq!(get_row(0), vec![1]);
    // assert_eq!(get_row(1), vec![1, 1]);
}
