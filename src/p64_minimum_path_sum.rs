struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![0; n];
        dp[0] = grid[0][0];
        for j in 1..n {
            dp[j] = dp[j - 1] + grid[0][j];
        }
        for i in 1..m {
            dp[0] += grid[i][0];
            for j in 1..n {
                dp[j] = dp[j - 1].min(dp[j]) + grid[i][j];
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }
}
