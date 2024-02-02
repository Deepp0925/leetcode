// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut merged = None;
    let mut merged_tail = &mut merged;

    loop {
        // both
        if list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                let mut node = list1.unwrap();
                list1 = node.next.take();
                merged_tail.replace(node);
                merged_tail = &mut merged_tail.as_mut().unwrap().next;
            } else {
                let mut node = list2.unwrap();
                list2 = node.next.take();
                merged_tail.replace(node);
                merged_tail = &mut merged_tail.as_mut().unwrap().next;
            }
        } else if list1.is_some() {
            merged_tail.replace(list1.take().unwrap());
            merged_tail = &mut merged_tail.as_mut().unwrap().next;
        } else if list2.is_some() {
            merged_tail.replace(list2.take().unwrap());
            merged_tail = &mut merged_tail.as_mut().unwrap().next;
        } else {
            break;
        }
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_lists() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let merged = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));

        assert_eq!(merge_two_lists(list1, list2), merged);
    }
}
