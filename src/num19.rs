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

pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut len = 0;
    let mut curr = &head;

    // get the length of the linked list
    while let Some(node) = curr {
        len += 1;
        curr = &node.next;
    }

    let mut curr = &mut head;
    let mut i = 0;

    if len == n {
        return head.unwrap().next;
    }

    while let Some(node) = curr {
        if i == len - n - 1 {
            node.next = node.next.as_mut().unwrap().next.take();
            break;
        }

        i += 1;
        curr = &mut node.next;
    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_nth_from_end() {
        assert_eq!(
            remove_nth_from_end(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode { val: 5, next: None })),
                            })),
                        })),
                    })),
                })),
                2
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            }))
        );
        assert_eq!(
            remove_nth_from_end(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode { val: 2, next: None })),
                })),
                1
            ),
            Some(Box::new(ListNode { val: 1, next: None }))
        );
        assert_eq!(
            remove_nth_from_end(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode { val: 2, next: None })),
                })),
                2
            ),
            Some(Box::new(ListNode { val: 2, next: None }))
        );
        assert_eq!(
            remove_nth_from_end(Some(Box::new(ListNode { val: 1, next: None })), 1),
            None
        );
        assert_eq!(remove_nth_from_end(None, 1), None);
    }
}
