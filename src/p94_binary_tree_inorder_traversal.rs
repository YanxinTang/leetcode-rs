use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::tree::TreeNode;

struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];

        fn traversal(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
            if root.is_none() {
                return;
            }
            let root = root.as_ref().unwrap().borrow();
            traversal(&root.left, ans);
            ans.push(root.val);
            traversal(&root.right, ans);
        }

        traversal(&root, &mut ans);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::inorder_traversal(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    }))),
                    right: None
                })))
            })))),
            vec![1, 3, 2]
        );
    }
}
