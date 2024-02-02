// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn swap_curr_next(node: &mut Option<Box<ListNode>>) -> bool {
        if node.is_none() {
            return false;
        }

        let mut curr = node.take().unwrap();
        if curr.next.is_none() {
            node.replace(curr);
            return false;
        }

        let mut next = curr.next.take().unwrap();
        let next2next = next.next.take();
        curr.next = next2next;
        next.next = Some(curr);
        node.replace(next);
        true
    }

    let mut current = &mut head;
    loop {
        if !swap_curr_next(current) {
            break;
        }
        // merged_tail = &mut merged_tail.as_mut().unwrap().next;
        current = &mut current.as_mut().unwrap().next.as_mut().unwrap().next;
    }

    head
}

#[test]
fn test_swap_pairs() {
    assert_eq!(
        swap_pairs(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }))),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        }))
    );

    assert_eq!(
        swap_pairs(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }))),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 1, next: None })),
        }))
    );

    assert_eq!(
        swap_pairs(Some(Box::new(ListNode { val: 1, next: None }))),
        Some(Box::new(ListNode { val: 1, next: None }))
    );
}
