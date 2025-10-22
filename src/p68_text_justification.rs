use std::vec;

struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width: usize = max_width as usize;
        let mut ans: Vec<String> = vec![];
        let mut row: Vec<String> = vec![];
        let mut row_words_len: usize = 0; // 最后一行中单词的长度，不包含空格
        for word in words {
            if row_words_len + row.len() + word.len() > max_width {
                let line = Self::justify_line(&row, max_width, false);
                ans.push(line);
                row = vec![];
                row_words_len = 0;
            }
            row_words_len += word.len();
            row.push(word);
        }
        // 最后一行
        ans.push(Self::justify_line(&row, max_width, true));
        ans
    }

    fn justify_line(words: &[String], max_width: usize, is_last: bool) -> String {
        if is_last || words.len() == 1 {
            let line = words.join(" ");
            let spaces_count = max_width - line.len();
            return line + &" ".repeat(spaces_count);
        }

        let mut line = String::new();
        let last_row_len = words.len();
        let word_len: usize = words.iter().map(|word| word.len()).sum();
        let gaps = last_row_len - 1;
        let total_spaces = max_width - word_len;
        // 单词间平均空格数
        let even_spaces = total_spaces / gaps;
        // 平均分之后，剩余的空格数
        let mut extra = total_spaces % gaps;

        for (i, item) in words.iter().enumerate() {
            line.push_str(item);
            if i < gaps {
                let extra_spaces_cnt = if extra > 0 {
                    extra -= 1;
                    1
                } else {
                    0
                };
                let spaces_cnt = even_spaces + extra_spaces_cnt;
                line.push_str(&" ".repeat(spaces_cnt));
            }
        }
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::full_justify(
                vec![
                    String::from("This"),
                    String::from("is"),
                    String::from("an"),
                    String::from("example"),
                    String::from("of"),
                    String::from("text"),
                    String::from("justification.")
                ],
                16
            ),
            vec!["This    is    an", "example  of text", "justification.  "]
        );

        assert_eq!(
            Solution::full_justify(
                vec![
                    String::from("What"),
                    String::from("must"),
                    String::from("be"),
                    String::from("acknowledgment"),
                    String::from("shall"),
                    String::from("be")
                ],
                16
            ),
            vec!["What   must   be", "acknowledgment  ", "shall be        "]
        );
        assert_eq!(
            Solution::full_justify(
                vec![
                    String::from("Science"),
                    String::from("is"),
                    String::from("what"),
                    String::from("we"),
                    String::from("understand"),
                    String::from("well"),
                    String::from("enough"),
                    String::from("to"),
                    String::from("explain"),
                    String::from("to"),
                    String::from("a"),
                    String::from("computer."),
                    String::from("Art"),
                    String::from("is"),
                    String::from("everything"),
                    String::from("else"),
                    String::from("we"),
                    String::from("do")
                ],
                20
            ),
            vec![
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            ]
        );
    }
}
