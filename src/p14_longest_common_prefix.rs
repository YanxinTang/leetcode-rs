struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix: Vec<char> = vec![];
        let s0 = &strs[0];

        let strs: Vec<Vec<char>> = strs.iter().map(|str| str.chars().collect()).collect();

        'outer: for (i, char_at) in s0.chars().enumerate() {
            for str in &strs[1..] {
                if let Some(&c) = str.get(i)
                    && c == char_at
                {
                } else {
                    break 'outer;
                }
            }
            prefix.push(char_at);
        }

        String::from_iter(prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ]),
            ""
        );
    }
}
