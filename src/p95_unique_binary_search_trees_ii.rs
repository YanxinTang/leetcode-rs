use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn dfs(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
            if start > end {
                return vec![None];
            }

            let mut trees: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
            for i in start..=end {
                let left_trees = dfs(start, i - 1);
                let right_trees = dfs(i + 1, end);

                for left in &left_trees {
                    for right in &right_trees {
                        let current_tree = Some(Rc::new(RefCell::new(TreeNode {
                            val: i,
                            left: left.clone(),
                            right: right.clone(),
                        })));
                        trees.push(current_tree);
                    }
                }
            }
            trees
        }
        dfs(1, n)
    }
}

#[cfg(test)]
mod tests {
    use crate::{bfs_vec, utils::tree::TreeNode};

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::generate_trees(3),
            vec![
                TreeNode::from_bfs(bfs_vec![1, null, 2, null, 3]),
                TreeNode::from_bfs(bfs_vec![1, null, 3, 2]),
                TreeNode::from_bfs(bfs_vec![2, 1, 3]),
                TreeNode::from_bfs(bfs_vec![3, 1, null, null, 2]),
                TreeNode::from_bfs(bfs_vec![3, 2, null, 1]),
            ]
        )
    }
}
