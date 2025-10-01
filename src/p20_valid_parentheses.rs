struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let mut pairs = std::collections::HashMap::<char, char>::with_capacity(3);
        pairs.insert(')', '(');
        pairs.insert(']', '[');
        pairs.insert('}', '{');

        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' => stack.push(ch),
                ')' | ']' | '}' => {
                    let left_pair = pairs.get(&ch).unwrap();
                    if let Some(last) = stack.last()
                        && last == left_pair
                    {
                        stack.pop();
                    } else {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
        assert_eq!(Solution::is_valid(String::from("(]")), false);
        assert_eq!(Solution::is_valid(String::from("([])")), true);
        assert_eq!(Solution::is_valid(String::from("([)]")), false);
        assert_eq!(Solution::is_valid(String::from("]")), false);
    }
}
