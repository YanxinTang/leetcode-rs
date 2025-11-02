use crate::utils::list::ListNode;

struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode { val: 0, next: head });
        let mut p = &mut dummy_head;

        while p.next.is_some() {
            let mut next = p.next.take().unwrap();
            let next_mut = next.as_mut();
            let mut duplicate = false;
            while let Some(next_next) = next_mut.next.as_ref() {
                if next_mut.val == next_next.val {
                    duplicate = true;
                    next_mut.next = next_mut.next.take().and_then(|n| n.next)
                } else {
                    break;
                }
            }
            if duplicate {
                p.next = next.next;
            } else {
                p.next = Some(next);
                p = p.next.as_mut().unwrap();
            }
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
            Solution::delete_duplicates(List::from_slice(&[1, 2, 3, 3, 4, 4, 5]).into()),
            List::from_slice(&[1, 2, 5]).into()
        );
    }
}
