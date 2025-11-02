use crate::utils::list::ListNode;

struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut head = head;
        let mut cur = head.as_mut().unwrap();
        while let Some(mut next) = cur.next.take() {
            if cur.val == next.val {
                cur.next = next.next.take();
            } else {
                cur.next = Some(next);
                cur = cur.next.as_mut().unwrap();
            }
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::list::List;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::delete_duplicates(List::from_slice(&[0, 0, 0, 0, 0]).into()),
            List::from_slice(&[0]).into()
        );
        assert_eq!(
            Solution::delete_duplicates(List::from_slice(&[1, 1, 2, 3, 3]).into()),
            List::from_slice(&[1, 2, 3]).into()
        );
        assert_eq!(
            Solution::delete_duplicates(List::from_slice(&[]).into()),
            List::from_slice(&[]).into()
        );
    }
}
