struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // 1. 排序
        let mut nums = nums;
        nums.sort();

        let mut ans: Vec<Vec<i32>> = vec![];
        let mut candidate: Vec<i32> = vec![];
        // 2. 回溯
        Self::dfs(&nums, &mut ans, &mut candidate, 0);
        ans
    }

    fn dfs(nums: &[i32], ans: &mut Vec<Vec<i32>>, candidate: &mut Vec<i32>, start: usize) {
        ans.push(candidate.clone());

        for i in start..nums.len() {
            // 重复元素剪枝
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            candidate.push(nums[i]);
            Self::dfs(nums, ans, candidate, i + 1);
            candidate.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::subsets_with_dup(vec![1, 2, 2]),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2]
            ]
        );
    }
}
