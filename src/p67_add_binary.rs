struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut ans: Vec<u8> = vec![];
        let mut iter_a = a.bytes().rev().peekable();
        let mut iter_b = b.bytes().rev().peekable();

        let mut carry: u8 = 0;
        while iter_a.peek().is_some() || iter_b.peek().is_some() {
            let da = iter_a.next().unwrap_or(b'0');
            let db = iter_b.next().unwrap_or(b'0');
            let sum = (da - b'0') + (db - b'0') + (carry);
            ans.push(sum % 2 + b'0');
            carry = sum / 2;
        }
        if carry > 0 {
            ans.push(carry + b'0');
        }
        ans.reverse();

        String::from_utf8(ans).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::add_binary(String::from("1"), String::from("1")),
            "10"
        );
        assert_eq!(
            Solution::add_binary(String::from("11"), String::from("1")),
            "100"
        );
        assert_eq!(
            Solution::add_binary(String::from("1010"), String::from("1011")),
            "10101"
        );
    }
}
