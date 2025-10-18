use crate::utils::list::ListNode;

struct Solution;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // 1. 找出链表长度 n
        let mut n = 0;
        let mut p = &head;
        while p.is_some() {
            p = &p.as_ref().unwrap().next;
            n += 1;
        }
        if n == 0 {
            return head;
        }
        // 2. k 对 n 取余得 m，截取末尾 m 个节点放在链表头，完成移动
        let m = k % n;
        if m == 0 {
            return head;
        }

        // 3. 末尾第 m 个，我们向后走 n - m 步
        let mut head = head;
        let mut slow = &mut head;
        for _ in 0..n - m - 1 {
            slow = &mut slow.as_mut().unwrap().next;
        }

        // 找到新头的末尾，接到旧头上
        let mut new_head = slow.as_mut().unwrap().next.take();
        let mut tail = &mut new_head;
        while tail.as_ref().unwrap().next.is_some() {
            tail = &mut tail.as_mut().unwrap().next;
        }
        tail.as_mut().unwrap().next = head;
        new_head
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::list::List;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::rotate_right(List::from_slice(&[1, 2, 3, 4, 5]).into(), 2),
            List::from_slice(&[4, 5, 1, 2, 3]).into()
        );
    }
}
