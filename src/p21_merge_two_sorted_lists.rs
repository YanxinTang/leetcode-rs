use crate::utils::list::ListNode;

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));

        let mut list1 = list1;
        let mut list2 = list2;
        let mut p = &mut dummy_head;

        while list1.is_some() && list2.is_some() {
            let mut n1 = list1.take().unwrap();
            let mut n2 = list2.take().unwrap();

            if n1.val < n2.val {
                list1 = n1.next.take();
                list2 = Some(n2);
                p.next = Some(n1);
            } else {
                list1 = Some(n1);
                list2 = n2.next.take();
                p.next = Some(n2);
            }
            p = p.next.as_mut().unwrap();
        }
        p.next = list1.or(list2);
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
            Solution::merge_two_lists(
                List::from_slice(&[1, 2, 4]).into(),
                List::from_slice(&[1, 3, 4]).into()
            ),
            List::from_slice(&[1, 1, 2, 3, 4, 4]).into()
        );
        assert_eq!(
            Solution::merge_two_lists(List::from_slice(&[]).into(), List::from_slice(&[]).into()),
            List::from_slice(&[]).into()
        );
        assert_eq!(
            Solution::merge_two_lists(List::from_slice(&[]).into(), List::from_slice(&[0]).into()),
            List::from_slice(&[0]).into()
        );
    }
}
