struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut ans = String::from("1");
        for _ in 2..=n {
            let ans_bytes = ans.as_bytes();
            let mut n_ans = String::new();
            let (mut l, mut r) = (0, 0);
            while r <= ans_bytes.len() {
                if r == ans_bytes.len() || ans_bytes[r] != ans_bytes[l] {
                    n_ans.push_str(&(r - l).to_string());
                    n_ans.push(ans_bytes[l] as char);
                    l = r;
                }
                r += 1;
            }
            ans = n_ans;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::count_and_say(4), "1211");
    }
}
