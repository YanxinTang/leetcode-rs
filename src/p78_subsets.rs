struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(nums: &[i32], start: usize, ans: &mut Vec<Vec<i32>>, candidate: &mut Vec<i32>) {
            ans.push(candidate.clone());
            for i in start..nums.len() {
                candidate.push(nums[i]);
                dfs(nums, i + 1, ans, candidate);
                candidate.pop();
            }
        }

        let mut ans: Vec<Vec<i32>> = vec![];
        let mut candidate: Vec<i32> = vec![];
        dfs(&nums, 0, &mut ans, &mut candidate);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
        assert_eq!(Solution::subsets(vec![0]), vec![vec![], vec![0]]);
    }
}
