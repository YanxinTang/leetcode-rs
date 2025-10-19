use std::vec;

struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        if m == 0 {
            return 0;
        }
        let n = obstacle_grid[0].len();
        if n == 0 {
            return 0;
        }
        let mut dp = vec![0; n];
        dp[0] = if obstacle_grid[0][0] == 1 { 0 } else { 1 };

        for i in 0..m {
            for j in 0..n {
                let cell = obstacle_grid[i][j];
                if cell == 1 {
                    dp[j] = 0;
                } else if j > 0 {
                    // dp[j] 从上方过来的路径数
                    // dp[j-1] 从左方过来的路径数
                    dp[j] += dp[j - 1];
                }
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
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1
        );
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1, 0]]), 0);
    }
}
