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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        let mut res = vec![];
        while !queue.is_empty() {
            let n = queue.len();

            for i in 0..n {
                let node = queue.pop_front().unwrap().unwrap();
                let node = node.borrow();

                if node.left.is_some() {
                    queue.push_back(node.left.clone());
                }

                if node.right.is_some() {
                    queue.push_back(node.right.clone());
                }

                // is the last item
                if i == n - 1 {
                    res.push(node.val);
                }
            }
        }

        res
    }
}

#[test]
fn test_fn() {
    // println!("{}", 5 / 2);
    // assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    // assert_eq!(get_row(0), vec![1]);
    // assert_eq!(get_row(1), vec![1, 1]);
}
