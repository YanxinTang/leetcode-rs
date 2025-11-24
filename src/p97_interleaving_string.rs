struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let s1 = s1.into_bytes();
        let s2 = s2.into_bytes();
        let s3 = s3.into_bytes();
        // dp[i][j] 表示 s1 前 i 个字符和 s2 前 j 个字符能交错组成 s3 前 i + j 个字符
        let mut dp: Vec<Vec<bool>> = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        dp[0][0] = true;

        for (i, &c1) in s1.iter().enumerate() {
            dp[i + 1][0] = dp[i][0] && c1 == s3[i]
        }
        for (j, &c2) in s2.iter().enumerate() {
            dp[0][j + 1] = dp[0][j] && c2 == s3[j]
        }

        for (i, &c1) in s1.iter().enumerate() {
            for (j, &c2) in s2.iter().enumerate() {
                let k = i + j + 1; // 当前 s3 的长度是 i + 1 + j + 1，索引是 i + j + 1
                dp[i + 1][j + 1] = (c1 == s3[k] && dp[i][j + 1]) || (c2 == s3[k] && dp[i + 1][j]);
            }
        }

        dp[s1.len()][s2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_interleave(
                String::from("aabcc"),
                String::from("dbbca"),
                String::from("aadbbcbcac")
            ),
            true,
        );
        assert_eq!(
            Solution::is_interleave(
                String::from("aabcc"),
                String::from("dbbca"),
                String::from("aadbbbaccc")
            ),
            false,
        );
        assert_eq!(
            Solution::is_interleave(String::from(""), String::from(""), String::from("")),
            true,
        );
        assert_eq!(
            Solution::is_interleave(String::from(""), String::from(""), String::from("a")),
            false,
        );
    }
}
