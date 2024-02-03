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

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    // extract all the numbers out
    let mut nums = vec![];
    let mut current = head;
    while let Some(node) = current {
        current = node.next;
        nums.push(node.val);
    }

    fn reverse_sub_array(nums: &mut [i32], range: std::ops::Range<usize>) {
        let mut left = range.start;
        let mut right = range.end;
        while left < right && right < nums.len() {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    for i in (0..nums.len()).step_by(k as usize) {
        reverse_sub_array(&mut nums, i..i + k as usize - 1);
    }

    let mut head = None;

    while let Some(num) = nums.pop() {
        head = Some(Box::new(ListNode {
            val: num,
            next: head,
        }));
    }

    head
}

#[test]
fn test_reverse_k_group() {
    let head1 = Some(Box::new(ListNode {
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
    }));

    assert_eq!(
        reverse_k_group(head1, 2),
        Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }))
    );

    let head2 = Some(Box::new(ListNode {
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
    }));

    assert_eq!(
        reverse_k_group(head2, 3),
        Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }))
    );
}
