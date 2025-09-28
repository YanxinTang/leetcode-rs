struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.chars().count() == 0 {
            return vec![];
        }
        let mut ret: Vec<String> = vec![String::from("")];
        for ch in digits.chars() {
            let arr: &[&str] = match ch {
                '2' => &["a", "b", "c"],
                '3' => &["d", "e", "f"],
                '4' => &["g", "h", "i"],
                '5' => &["j", "k", "l"],
                '6' => &["m", "n", "o"],
                '7' => &["p", "q", "r", "s"],
                '8' => &["t", "u", "v"],
                '9' => &["w", "x", "y", "z"],
                _ => &[],
            };

            ret = ret
                .iter()
                .flat_map(|item| arr.iter().map(|&s| item.to_string() + s))
                .collect::<Vec<String>>()
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::letter_combinations(String::from("23")),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations(String::from("")),
            Vec::<String>::with_capacity(0)
        );
        assert_eq!(
            Solution::letter_combinations(String::from("2")),
            vec!["a", "b", "c"]
        );
    }
}
