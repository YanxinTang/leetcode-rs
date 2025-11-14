struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        if bytes.is_empty() {
            return 0;
        }
        let mut dp: Vec<i32> = vec![0; s.len() + 1];
        dp[0] = 1; // 占位
        dp[1] = if bytes[0] == b'0' { 0 } else { 1 };
        for i in 2..=bytes.len() {
            let c1 = bytes[i - 1];
            let c2 = bytes[i - 2];
            if c1 - b'0' > 0 {
                dp[i] += dp[i - 1]
            }
            let num = (c2 - b'0') * 10 + c1 - b'0';
            if (10..=26).contains(&num) {
                dp[i] += dp[i - 2]
            }
            if dp[i] == 0 {
                break;
            }
        }

        dp[dp.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::num_decodings(String::from("1201234")), 3);
        assert_eq!(Solution::num_decodings(String::from("2101")), 1);
        assert_eq!(Solution::num_decodings(String::from("12")), 2);
        assert_eq!(Solution::num_decodings(String::from("226")), 3);
        assert_eq!(Solution::num_decodings(String::from("06")), 0);
    }
}
