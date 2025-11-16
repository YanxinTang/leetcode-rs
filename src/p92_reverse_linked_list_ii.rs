use crate::utils::list::ListNode;

struct Solution;

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode { val: 0, next: head };
        let mut pre = &mut dummy_head;

        for _ in 1..left {
            pre = pre.next.as_mut().unwrap();
        }

        let mut curr = pre.next.take(); // tail = Some(left)
        let mut prev = None; // 新的头节点
        for _ in left..=right {
            if let Some(mut node) = curr {
                curr = node.next.take(); // curr 后移
                node.next = prev;
                prev = Some(node);
            }
        }

        pre.next = prev;

        // 找到新的尾，把 curr 接上
        while pre.next.is_some() {
            pre = pre.next.as_mut().unwrap();
        }
        pre.next = curr;
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
            Solution::reverse_between(List::from_slice(&[1, 2, 3, 4, 5]).into(), 2, 4),
            List::from_slice(&[1, 4, 3, 2, 5]).into()
        );
        // assert_eq!(
        //     Solution::reverse_between(List::from_slice(&[5]).into(), 1, 1),
        //     List::from_slice(&[5]).into()
        // );
    }
}
