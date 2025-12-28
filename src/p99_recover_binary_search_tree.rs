struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        fn inorder_tranversal(
            root: &Option<Rc<RefCell<TreeNode>>>,
            prev: &mut Option<Rc<RefCell<TreeNode>>>,
            x: &mut Option<Rc<RefCell<TreeNode>>>,
            y: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(root_node) = root {
                let left = root_node.borrow().left.clone();
                inorder_tranversal(&left, prev, x, y);
                if let Some(prev_node) = prev
                    && prev_node.borrow().val > root_node.borrow().val
                {
                    if x.is_none() {
                        *x = Some(prev_node.clone())
                    }
                    *y = Some(root_node.clone())
                }
                *prev = Some(root_node.clone());
                let right = root_node.borrow().right.clone();
                inorder_tranversal(&right, prev, x, y);
            }
        }

        let x: &mut Option<Rc<RefCell<TreeNode>>> = &mut None;
        let y: &mut Option<Rc<RefCell<TreeNode>>> = &mut None;
        inorder_tranversal(root, &mut None, x, y);

        if let (Some(x_node), Some(y_node)) = (x, y) {
            let mut x_ref = x_node.borrow_mut();
            let mut y_ref = y_node.borrow_mut();
            std::mem::swap(&mut x_ref.val, &mut y_ref.val);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bfs_vec, utils::tree::TreeNode};

    use super::*;

    #[test]
    fn test() {
        {
            let mut tree = TreeNode::from_bfs(bfs_vec![1, 3, null, null, 2]);
            Solution::recover_tree(&mut tree);
            assert_eq!(tree, TreeNode::from_bfs(bfs_vec![3, 1, null, null, 2]));
        }
        {
            let mut tree = TreeNode::from_bfs(bfs_vec![3, 1, 4, null, null, 2]);
            Solution::recover_tree(&mut tree);
            assert_eq!(tree, TreeNode::from_bfs(bfs_vec![2, 1, 4, null, null, 3]));
        }
    }
}
