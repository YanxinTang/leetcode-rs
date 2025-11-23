use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl TreeNode {
    /// 从 BFS 数组 (LeetCode 格式) 构建二叉树
    /// 输入: vec![Some(1), None, Some(2), Some(3)]
    pub fn from_bfs(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if vec.is_empty() {
            return None;
        }

        // 1. 处理根节点
        // 如果第一个元素是 None，直接返回 None
        let root_val = vec[0]?;
        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));

        // 2. 初始化队列，放入根节点
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        // 3. 使用迭代器遍历剩余的数组元素 (跳过 root)
        let mut iter = vec.into_iter().skip(1);

        // 当队列不为空，且数组里还有元素时循环
        while let Some(parent) = queue.pop_front() {
            // --- 处理左孩子 ---
            if let Some(val_opt) = iter.next() {
                if let Some(val) = val_opt {
                    let left_node = Rc::new(RefCell::new(TreeNode::new(val)));
                    // 链接到父节点
                    parent.borrow_mut().left = Some(Rc::clone(&left_node));
                    // 只有非空节点才入队
                    queue.push_back(left_node);
                }
            } else {
                break; // 数组已空
            }

            // --- 处理右孩子 ---
            if let Some(val_opt) = iter.next() {
                if let Some(val) = val_opt {
                    let right_node = Rc::new(RefCell::new(TreeNode::new(val)));
                    // 链接到父节点
                    parent.borrow_mut().right = Some(Rc::clone(&right_node));
                    // 只有非空节点才入队
                    queue.push_back(right_node);
                }
            } else {
                break; // 数组已空
            }
        }

        Some(root)
    }

    /// 将二叉树序列化为 BFS 数组 (LeetCode 格式)
    pub fn to_bfs_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
        let mut result: Vec<Option<i32>> = Vec::new();
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

        if root.is_none() {
            return result; // 空树返回空数组 []
        }

        // 队列中存储的是 Option<Rc<RefCell<TreeNode>>>
        queue.push_back(root);

        while !queue.is_empty() {
            // 弹出当前节点
            if let Some(node_opt) = queue.pop_front().flatten() {
                let node = node_opt.borrow();

                // 1. 记录值
                result.push(Some(node.val));

                // 2. 将子节点（可能为 None）推入队列
                // 使用 .as_ref().map(Rc::clone) 优雅地处理 Option 和 Rc 的引用
                queue.push_back(node.left.as_ref().map(Rc::clone));
                queue.push_back(node.right.as_ref().map(Rc::clone));
            } else {
                // 如果弹出的是 None (即 queue.pop_front().flatten() 为 None)
                result.push(None);
                // 此时是空节点，无需将其子节点推入队列
            }
        }

        // 3. 清理尾部的 None (LeetCode 序列化规则)
        // 找到最后一个非 None 元素的索引
        let mut trim_len = result.len();
        while trim_len > 0 && result[trim_len - 1].is_none() {
            trim_len -= 1;
        }
        result.truncate(trim_len);

        result
    }
}
