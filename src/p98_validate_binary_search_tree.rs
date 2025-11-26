use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;
struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            lower: Option<i32>,
            upper: Option<i32>,
        ) -> bool {
            if root.is_none() {
                return true;
            }
            let root = root.as_ref().unwrap().borrow();
            let root_val = root.val;
            if lower.is_some_and(|l| root_val <= l) || upper.is_some_and(|u| root_val >= u) {
                return false;
            }

            dfs(root.left.clone(), lower, Some(root_val))
                && dfs(root.right.clone(), Some(root_val), upper)
        }

        dfs(root, None, None)
    }
}

#[cfg(test)]
mod tests {
    use crate::bfs_vec;

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_bfs(bfs_vec![2, 1, 3])),
            true
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_bfs(bfs_vec![5, 1, 4, null, null, 3, 6])),
            false
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_bfs(bfs_vec![5, 4, 6, null, null, 3, 7])),
            false
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_bfs(bfs_vec![2147483647])),
            true
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_bfs(bfs_vec![2147483647, 2147483647])),
            false
        );
        assert_eq!(
            Solution::is_valid_bst(TreeNode::from_bfs(vec![
                Some(-2147483647),
                None,
                Some(2147483647)
            ])),
            true
        );
    }
}
