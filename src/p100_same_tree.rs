use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let p_node = p.borrow();
                let q_node = q.borrow();
                if p_node.val != q_node.val {
                    return false;
                }
                Self::is_same_tree(p_node.left.clone(), q_node.left.clone())
                    && Self::is_same_tree(p_node.right.clone(), q_node.right.clone())
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bfs_vec, utils::tree::TreeNode};

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_same_tree(
                TreeNode::from_bfs(bfs_vec![1, 2, 3]),
                TreeNode::from_bfs(bfs_vec![1, 2, 3])
            ),
            true,
        );
        assert_eq!(
            Solution::is_same_tree(
                TreeNode::from_bfs(bfs_vec![1, 2]),
                TreeNode::from_bfs(bfs_vec![1, null, 2])
            ),
            false,
        );
        assert_eq!(
            Solution::is_same_tree(
                TreeNode::from_bfs(bfs_vec![1, 2, 1]),
                TreeNode::from_bfs(bfs_vec![1, 1, 2])
            ),
            false,
        );
    }
}
