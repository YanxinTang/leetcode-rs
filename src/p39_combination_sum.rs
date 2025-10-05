struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(
            candidates: &[i32],
            target: i32,
            ans: &mut Vec<Vec<i32>>,
            combine: &mut Vec<i32>,
            idx: usize,
        ) {
            if idx == candidates.len() {
                return;
            }
            if target == 0 {
                ans.push(combine.to_vec());
                return;
            }
            dfs(candidates, target, ans, combine, idx + 1);
            if target - candidates[idx] >= 0 {
                combine.push(candidates[idx]);
                dfs(candidates, target - candidates[idx], ans, combine, idx);
                combine.pop();
            }
        }

        let mut ans: Vec<Vec<i32>> = vec![];
        let mut combile: Vec<i32> = vec![];
        dfs(&candidates, target, &mut ans, &mut combile, 0);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![7], vec![2, 2, 3]]
        )
    }
}
