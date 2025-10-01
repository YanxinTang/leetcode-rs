use crate::utils::list::ListNode;

struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut fast = &dummy_head.clone();
        let mut slow = &mut dummy_head;

        for _ in 0..=n {
            fast = &fast.as_ref().unwrap().next;
        }

        while let Some(node) = fast {
            fast = &node.next;
            slow = &mut slow.as_mut().unwrap().next
        }

        let removing_node = &mut slow.as_mut().unwrap().next;
        slow.as_mut().unwrap().next = removing_node.as_mut().unwrap().next.take();

        dummy_head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::list::List;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::remove_nth_from_end(List::from_slice(&[1, 2, 3, 4, 5]).into(), 2),
            List::from_slice(&[1, 2, 3, 5]).into()
        );
        assert_eq!(
            Solution::remove_nth_from_end(List::from_slice(&[1]).into(), 1),
            None
        );
        assert_eq!(
            Solution::remove_nth_from_end(List::from_slice(&[1, 2]).into(), 1),
            List::from_slice(&[1]).into()
        );
    }
}
