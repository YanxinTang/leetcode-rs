struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.split_ascii_whitespace()
            .last()
            .unwrap_or_default()
            .chars()
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::length_of_last_word(String::from("Hello World")),
            5
        );
    }
}
