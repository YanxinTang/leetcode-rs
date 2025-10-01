struct Solution;

impl Solution {
    fn back_trace(ans: &mut Vec<String>, cur: &mut String, open: usize, close: usize, max: usize) {
        if cur.len() == max * 2 {
            ans.push(cur.to_string());
            return;
        }

        if open < max {
            cur.push('(');
            Self::back_trace(ans, cur, open + 1, close, max);
            cur.pop();
        }

        if close < open {
            cur.push(')');
            Self::back_trace(ans, cur, open, close + 1, max);
            cur.pop();
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = vec![];
        let mut cur = String::from("");
        Self::back_trace(&mut ans, &mut cur, 0, 0, n as usize);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        )
    }
}
