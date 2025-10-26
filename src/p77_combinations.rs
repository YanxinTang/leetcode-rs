struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn dfs(start: i32, end: i32, k: usize, ans: &mut Vec<Vec<i32>>, candidate: &mut Vec<i32>) {
            if candidate.len() == k {
                ans.push(candidate.clone());
                return;
            }
            for item in start..=end {
                candidate.push(item);
                dfs(item + 1, end, k, ans, candidate);
                candidate.pop();
            }
        }
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut candidate: Vec<i32> = vec![];
        dfs(1, n, k as usize, &mut ans, &mut candidate);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ]
        );
        assert_eq!(Solution::combine(1, 1), vec![vec![1],]);
    }
}
