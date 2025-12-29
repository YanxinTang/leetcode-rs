use crate::utils::tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn is_symmetric(
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(p), Some(q)) => {
                    let p = p.borrow();
                    let q = q.borrow();
                    if p.val != q.val {
                        return false;
                    }
                    is_symmetric(p.left.clone(), q.right.clone())
                        && is_symmetric(p.right.clone(), q.left.clone())
                }
                _ => false,
            }
        }

        if root.is_none() {
            return true;
        }

        let root = root.unwrap();
        let root = root.borrow();
        is_symmetric(root.left.clone(), root.right.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::bfs_vec;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_symmetric(TreeNode::from_bfs(bfs_vec![1, 2, 2, 3, 4, 4, 3])),
            true
        );
        assert_eq!(
            Solution::is_symmetric(TreeNode::from_bfs(bfs_vec![1, 2, 2, null, 3, null, 3])),
            false
        );
    }
}
