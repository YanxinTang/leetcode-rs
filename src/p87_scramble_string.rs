struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn is_scramble(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }

        // s1 中 i 开始长度为 len 的字符串是否能变换为从字符串 s2 中 j 开始长度为 len 的字符串
        let n = s1.len();
        let mut dp: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; n + 1]; n]; n];
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        for i in 0..n {
            for j in 0..n {
                if s1[i] == s2[j] {
                    dp[i][j][1] = true;
                }
            }
        }
        for len in 2..=n {
            for i in 0..=n - len {
                for j in 0..=n - len {
                    for k in 1..len {
                        // 不交换, s1[i..i+k]^s2[j..j+k] ^s1[i+k..len] ^ s2[j+k..len]
                        if dp[i][j][k] && dp[i + k][j + k][len - k] {
                            dp[i][j][len] = true;
                            break;
                        }
                        // 交换 s1[i..i+k] ^ s2[len-k-1..len] ^ s1[i+k..len] ^ s2[j..len-k-1]
                        if dp[i][j + len - k][k] && dp[i + k][j][len - k] {
                            dp[i][j][len] = true;
                            break;
                        }
                    }
                }
            }
        }
        dp[0][0][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_scramble(String::from("abc"), String::from("acb")),
            true,
        );
        assert_eq!(
            Solution::is_scramble(String::from("abcdbdacbdac"), String::from("bdacabcdbdac")),
            true,
        );
        assert_eq!(
            Solution::is_scramble(String::from("great"), String::from("rgeat")),
            true,
        );
        assert_eq!(
            Solution::is_scramble(String::from("abcde"), String::from("caebd")),
            false,
        );
        assert_eq!(
            Solution::is_scramble(String::from("a"), String::from("a")),
            true,
        );
    }
}
