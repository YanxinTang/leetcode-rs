struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // dp[i][j] 表示从 word1(0..i] 变换到 word2(0..j] 需要的最小变换次数
        let mut dp: Vec<Vec<i32>> = vec![vec![0; word2.len() + 1]; word1.len() + 1];

        // 1. 初始条件 dp[0][j] = j; dp[i][0] = i;
        dp.iter_mut()
            .enumerate()
            .for_each(|(i, row)| row[0] = i as i32);
        dp.first_mut()
            .unwrap()
            .iter_mut()
            .enumerate()
            .for_each(|(j, cell)| *cell = j as i32);

        // 2. 变换
        // 如果 word[i] == word[j]: dp[i][j] = dp[i-1][j-1]
        // 如果 word[i] != word[j]:
        // dp[i][j] = 1 + min(
        //   dp[i-1][j], // word1 插入一个字符
        //   dp[i][j-1], // word2 插入一个字符
        //   dp[i][j],   // word1 和 word2 的最后一个字符互相替换
        // )
        for (i, c1) in word1.bytes().enumerate() {
            // dp[0] 表示没有字符，并不是word1 第一个字符
            // word1[i] 到 word2[j] 是放在 dp[i + 1][j + 1] 中的
            let i = i + 1;
            for (j, c2) in word2.bytes().enumerate() {
                let j = j + 1;
                if c1 == c2 {
                    dp[i][j] = dp[i - 1][j - 1];
                    continue;
                }
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
            }
        }
        dp[word1.len()][word2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::min_distance(String::from("horse"), String::from("ros")),
            3
        );
        assert_eq!(
            Solution::min_distance(String::from("intention"), String::from("execution")),
            5
        );
        assert_eq!(
            Solution::min_distance(String::from(""), String::from("")),
            0
        );
    }
}
