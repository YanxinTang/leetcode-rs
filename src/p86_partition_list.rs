use crate::utils::list::ListNode;

struct Solution;

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode { val: 0, next: head };
        let mut ans_dummy_head = ListNode { val: 0, next: None };
        let mut p = &mut dummy_head; // 原始链表
        let mut ans_p = &mut ans_dummy_head; // 处理后链表

        while let Some(ref next) = p.next.as_mut() {
            if next.val < x {
                ans_p.next = p.next.take();
                ans_p = ans_p.next.as_mut().unwrap();
                p.next = ans_p.next.take();
            } else {
                p = p.next.as_mut().unwrap();
            }
        }
        ans_p.next = dummy_head.next.take();
        ans_dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::list::List;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::partition(List::from_slice(&[1, 4, 3, 2, 5, 2]).into(), 3),
            List::from_slice(&[1, 2, 2, 4, 3, 5]).into()
        );
        assert_eq!(
            Solution::partition(List::from_slice(&[2, 1]).into(), 2),
            List::from_slice(&[1, 2]).into()
        );
    }
}
