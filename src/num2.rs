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
pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut q1 = Vec::with_capacity(100);
    let mut q2 = Vec::with_capacity(100);

    while let Some(node) = l1 {
        q1.push(node.val);
        l1 = node.next;
    }

    while let Some(node) = l2 {
        q2.push(node.val);
        l2 = node.next;
    }

    let mut q1 = q1.into_iter();
    let mut q2 = q2.into_iter();

    let mut carry = 0;
    let mut res = None;

    while q1.len() != 0 || q2.len() != 0 || carry != 0 {
        let mut sum = carry;

        if let Some(val) = q1.next() {
            sum += val;
        }

        if let Some(val) = q2.next() {
            sum += val;
        }

        carry = sum / 10;
        let mut node = ListNode::new(sum % 10);
        node.next = res;
        res = Some(Box::new(node));
    }

    // reverse the list
    let mut prev = None;

    while let Some(mut node) = res {
        let next = node.next.take();
        node.next = prev;
        prev = Some(node);
        res = next;
    }

    prev
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_two_numbers() {
        use super::*;
        let mut l1 = ListNode::new(2);
        let mut l2 = ListNode::new(5);
        l1.next = Some(Box::new(ListNode::new(4)));
        l1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        l2.next = Some(Box::new(ListNode::new(6)));
        l2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

        let mut l3 = ListNode::new(7);
        l3.next = Some(Box::new(ListNode::new(0)));
        l3.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(8)));

        assert_eq!(
            add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))),
            Some(Box::new(l3))
        );

        let mut l1 = ListNode::new(9);
    }
}
