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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut v = vec![];
        let mut queue = VecDeque::new();
        if let Some(root) = root {
            queue.push_back(root);
        }

        while !queue.is_empty() {
            let n = queue.len();
            let mut temp = Vec::with_capacity(n);

            for _ in 0..n {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();

                temp.push(node.val);

                if let Some(left) = &node.left {
                    queue.push_back(left.clone());
                }

                if let Some(right) = &node.right {
                    queue.push_back(right.clone());
                }
            }

            v.push(temp);
        }

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
