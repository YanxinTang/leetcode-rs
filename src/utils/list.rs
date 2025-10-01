//! 单向链表的实现

use std::fmt::Display;

/// Definition for singly-linked list node.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)?;
        let mut p = self.next.as_ref();
        while let Some(node) = p {
            write!(f, " -> {}", node.val)?;
            p = node.next.as_ref();
        }
        Ok(())
    }
}

impl From<List> for Option<Box<ListNode>> {
    fn from(value: List) -> Self {
        value.0
    }
}

/// Definition for singly-linked list.
#[derive(Debug, PartialEq, Eq)]
pub struct List(pub Option<Box<ListNode>>);

impl List {
    pub fn from_slice(vals: &[i32]) -> List {
        let mut h = None;
        let mut p = &mut h;
        for &val in vals {
            let node = Box::new(ListNode::new(val));
            *p = Some(node);
            p = &mut p.as_mut().unwrap().next;
        }
        List(h)
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(node) = &self.0 {
            write!(f, "{}", node)
        } else {
            write!(f, "{{}}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_list() {
        let list = List::from_slice(&[1, 2, 3]);
        assert_eq!(format!("{}", list), "1 -> 2 -> 3")
    }

    #[test]
    fn list_comparison() {
        let l_a = List::from_slice(&[1, 2, 3]);
        let l_b = List::from_slice(&[1, 2, 3]);
        let l_c = List::from_slice(&[1, 2, 3, 4]);
        assert_eq!(l_a, l_b);
        assert_ne!(l_a, l_c);
    }
}
