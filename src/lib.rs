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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut v = vec![];
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, curr_str: &mut String, v: &mut Vec<String>) {
            if node.is_none() {
                return;
            }

            let node = node.as_ref().unwrap().borrow();

            if node.left.is_none() && node.right.is_none() {
                let mut c = curr_str.clone();
                c.push_str(&node.val.to_string());
                v.push(c);
                return;
            }

            curr_str.push_str(&node.val.to_string());
            dfs(node.left.clone(), curr_str, v);
            dfs(node.right.clone(), curr_str, v);
            curr_str.pop();
        }

        dfs(root, &mut String::new(), &mut v);

        v.into_iter()
            .fold(0, |acc, e| acc + e.parse::<i32>().unwrap())
    }
}

#[test]
fn test_fn() {
    // println!("{}", 5 / 2);
    // assert_eq!(get_row(3), vec![1, 3, 3, 1]);
    // assert_eq!(get_row(0), vec![1]);
    // assert_eq!(get_row(1), vec![1, 1]);
}
