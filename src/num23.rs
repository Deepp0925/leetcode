#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut merged: Option<Box<ListNode>> = None;
    let mut merged_tail = &mut merged;

    loop {
        let mut min = None;
        let mut min_index = 0;

        // determine which has the smallest index
        for (i, node) in lists.iter().enumerate() {
            if let Some(item) = node {
                if let Some(curr_min) = min {
                    if curr_min <= item.val {
                        continue;
                    }
                }
                min = Some(item.val);
                min_index = i;
            }
        }

        if min.is_none() {
            break;
        }

        let mut node = lists[min_index].take().unwrap();
        lists[min_index] = node.next.take();
        if merged_tail.is_none() {
            merged_tail.replace(node);
            continue;
        }

        merged_tail.as_mut().unwrap().next = Some(node);
        merged_tail = &mut merged_tail.as_mut().unwrap().next;
    }

    merged
}

#[test]
fn test_23() {
    assert_eq!(merge_k_lists(vec![]), None);
    assert_eq!(
        merge_k_lists(vec![
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            })),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 6, next: None }))
            }))
        ]),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode { val: 6, next: None }))
                                }))
                            }))
                        }))
                    }))
                }))
            }))
        }))
    );
}
