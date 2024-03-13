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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut paths = vec![];

        fn dfs(
            tree: Option<Rc<RefCell<TreeNode>>>,
            curr_path: &mut Vec<i32>,
            paths: &mut Vec<Vec<i32>>,
            target_sum: i32,
        ) {
            if let Some(tree) = tree {
                let tree = tree.borrow();
                curr_path.push(tree.val);
                // encountered a leaf
                if tree.left.is_none() && tree.right.is_none() && target_sum - tree.val == 0 {
                    paths.push(curr_path.clone())
                }

                dfs(tree.left.clone(), curr_path, paths, target_sum - tree.val);
                dfs(tree.right.clone(), curr_path, paths, target_sum - tree.val);

                curr_path.pop();
            }
        }

        dfs(root, &mut Vec::new(), &mut paths, target_sum);

        paths
    }
}

#[test]
fn test_fn() {
    // println!("{}", 5 / 2);
    // assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    // assert_eq!(get_row(0), vec![1]);
    // assert_eq!(get_row(1), vec![1, 1]);
}
