struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // 到最后一个格子，有两条路，从上到下，或者从左到右
        // 因此分解成子问题了
        // f(m, n) = f(m, n-1) + f(m-1, n)
        // f(1, 1) = 1;
        // f(1, 2) = 1; f(1, 3) = 1; f(1, n) = 1;
        // f(2, 1) = 1; f(3, 1) = 1; f(m, 1) = 1;
        // 递归超时，动态规则？
        // dp[1][n] = 1; dp[m][1] = 1;
        // dp[m][n] = dp[m-1][n] + dp[m][n-1];
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n]; m];
        dp[0] = vec![1; n];
        for i in 0..m {
            dp[i][0] = 1;
        }
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = dp[i][j - 1] + dp[i - 1][j];
            }
        }

        dp[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }
}
