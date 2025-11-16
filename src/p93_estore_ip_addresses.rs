struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn dfs(
            s: &[u8],
            ans: &mut Vec<String>,
            candidate: &mut Vec<u8>,
            start: usize,
            level: usize,
        ) {
            if level > 4 {
                return;
            }
            if start == s.len() {
                if level == 4 {
                    unsafe { ans.push(String::from_utf8_unchecked(candidate.clone())) }
                }
                return;
            }

            for i in 1..=3 {
                if start + i > s.len() {
                    break;
                }
                if i > 1 && s[start] == b'0' {
                    return;
                }
                if i == 3 {
                    let sum: u16 = s[start..start + i]
                        .iter()
                        .fold(0, |acc, &item| acc * 10 + (item - b'0') as u16);
                    if sum > 255 {
                        return;
                    }
                }

                candidate.extend_from_slice(&s[start..start + i]);
                if level < 3 {
                    candidate.push(b'.');
                }

                dfs(s, ans, candidate, start + i, level + 1);

                if level < 3 {
                    candidate.pop();
                }
                candidate.truncate(candidate.len() - i);
            }
        }

        let mut ans: Vec<String> = vec![];
        let mut candidate: Vec<u8> = vec![];
        dfs(s.as_bytes(), &mut ans, &mut candidate, 0, 0);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::restore_ip_addresses(String::from("25525511135")),
            vec!["255.255.11.135", "255.255.111.35"]
        );
        assert_eq!(
            Solution::restore_ip_addresses(String::from("0000")),
            vec!["0.0.0.0"]
        );
        assert_eq!(
            Solution::restore_ip_addresses(String::from("101023")),
            vec![
                "1.0.10.23",
                "1.0.102.3",
                "10.1.0.23",
                "10.10.2.3",
                "101.0.2.3"
            ]
        );
    }
}
