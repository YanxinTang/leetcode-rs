use crate::utils::list::ListNode;

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode { val: 0, next: head };
        // let mut head = head;
        let mut p = &mut dummy_head;
        while p.next.is_some() && p.next.as_ref().unwrap().next.is_some() {
            // p -> next -> next_next ->
            let mut next = p.next.take();
            let mut next_next = next.as_mut().unwrap().next.take();

            next.as_mut().unwrap().next = next_next.as_mut().unwrap().next.take();
            next_next.as_mut().unwrap().next = next;
            p.next = next_next;

            // next, next_next 所有权已经转移，无法直接访问
            p = p.next.as_mut().unwrap().next.as_mut().unwrap();
        }
        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::list::List;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::swap_pairs(List::from_slice(&[1, 2, 3, 4]).into()),
            List::from_slice(&[2, 1, 4, 3]).into()
        );
    }
}
