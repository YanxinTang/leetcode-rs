struct Solution;

impl Solution {
    // dp[n] = dp[n-1] + dp[n-2]
    // dp[1] = 1
    // dp[2] = 2
    // dp[3] = dp[1] + dp[2]
    // dp[4] = dp[3] + dp[2]
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 2 {
            return n;
        }
        let (mut a, mut b) = (1, 2);
        let n = n as usize;
        for _ in 3..=n {
            (a, b) = (b, a + b)
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
