use std::vec;

struct Solution;

impl Solution {
    /*
     * 动态规划
     *
     * d[i][j] = s[i] == s[j] && d[i+1][j-1]
     */
    pub fn longest_palindrome(s: String) -> String {
        if s.chars().count() <= 2 {
            return s;
        };

        let s: Vec<char> = s.chars().collect();
        let mut longest: &[char] = &s[0..=0];

        let mut dp = vec![vec![false; s.len()]; s.len()];
        dp.iter_mut().enumerate().for_each(|(i, row)| row[i] = true);

        for i in 0..s.len() - 1 {
            if s[i] == s[i + 1] {
                dp[i][i + 1] = true;
                longest = &s[i..=i + 1];
            }
        }
        for diff in 2..s.len() {
            for i in 0..s.len() - diff {
                let j = i + diff;
                if s[i] == s[j] && dp[i + 1][j - 1] {
                    dp[i][j] = true;
                    if longest.len() < diff + 1 {
                        longest = &s[i..=j];
                    }
                }
            }
        }
        String::from_iter(longest)
    }

    /*
     * 暴力枚举
     */
    pub fn longest_palindrome1(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut longest: Vec<char> = vec![];

        for i in 0..s.len() {
            for j in i + 1..s.len() {
                if j - i == 0 && longest.len() < j - i + 1 {
                    longest = s[i..=j].to_vec();
                    continue;
                }

                if j - i == 1 && s[i] == s[j] && longest.len() < j - i + 1 {
                    longest = s[i..=j].to_vec();
                    continue;
                }

                if Solution::is_palindrome(s[i..=j].to_vec()) && longest.len() < j - i + 1 {
                    longest = s[i..=j].to_vec();
                }
            }
        }

        longest.into_iter().collect()
    }

    fn is_palindrome(s: Vec<char>) -> bool {
        for k in 0..s.len() / 2 {
            if s[k] == s[s.len() - 1 - k] {
                continue;
            }
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::longest_palindrome(String::from("babad")),
            String::from("bab")
        );
        assert_eq!(Solution::longest_palindrome(String::from("cbbd")), "bb");
        assert_eq!(Solution::longest_palindrome(String::from("abb")), "bb");
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::longest_palindrome1(String::from("babad")),
            String::from("bab")
        );
        assert_eq!(Solution::longest_palindrome1(String::from("cbbd")), "bb");
        assert_eq!(Solution::longest_palindrome1(String::from("abb")), "bb");
        assert_eq!(Solution::longest_palindrome1(String::from("")), "");
        assert_eq!(Solution::longest_palindrome1(String::from("ccc")), "ccc");
        assert_eq!(
            Solution::longest_palindrome1(String::from("aaaaa")),
            "aaaaa"
        );
    }
}
