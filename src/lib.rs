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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut res = root.clone();
        // returning format has to

        // p should the smaller than q
        fn bst(root: OptNode, p: OptNode, q: OptNode, res: &mut OptNode) {
            if root.is_none() {
                return;
            }
            // check if it is equal to current val
            let root = root.unwrap();
            let val = root.borrow().val;
            let p_val = p.as_ref().unwrap().borrow().val;
            let q_val = q.as_ref().unwrap().borrow().val;
            // one of the val matches
            if val == p_val || val == q_val {
                *res = Some(root);
                return;
            }

            // none of the values match
            // is p val is less than curr val and q val is greater than val
            match (val > p_val, val < q_val) {
                // if so then current node is common
                (true, true) => {
                    *res = Some(root);
                }
                // both values are in the left sub tree
                (true, false) => bst(root.borrow().left.clone(), p, q, res),
                // both values are in the right sub tree
                (false, true) => bst(root.borrow().right.clone(), p, q, res),
                // traversing the wrong tree
                // should almost never happen
                (false, false) => unreachable!(),
            }
        }

        let (p, q) = if p.as_ref().unwrap().borrow().val < q.as_ref().unwrap().borrow().val {
            (p, q)
        } else {
            (q, p)
        };

        bst(root, p, q, &mut res);

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
