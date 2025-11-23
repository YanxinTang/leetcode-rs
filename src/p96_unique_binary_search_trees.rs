use std::vec;

struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        let mut dp: Vec<i32> = vec![0; n + 1];
        dp[0] = 1;
        if n >= 1 {
            dp[1] = 1;
        }

        // n = i 时求解二叉搜索树的个数
        for i in 2..=n {
            for j in 1..=i {
                // 第 j 位数字时，左边子树长 j-1，右边子树长 i - j
                dp[i] += dp[j - 1] * dp[i - j]
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::num_trees(3), 5);
        assert_eq!(Solution::num_trees(1), 1);
    }
}
