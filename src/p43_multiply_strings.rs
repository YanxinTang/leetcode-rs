struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1 = num1.as_bytes();
        let num2 = num2.as_bytes();
        let zero = b'0';
        let mut sum: Vec<u8> = vec![0; num1.len() + num2.len()];
        let mut addition = 0;
        for (i1, d2) in num1.iter().rev().enumerate() {
            for (i2, d1) in num2.iter().rev().enumerate() {
                let multiplex = (d1 - zero) * (d2 - zero) + addition + sum[i1 + i2];
                addition = multiplex / 10;
                sum[i1 + i2] = multiplex % 10;
            }
            if addition > 0 {
                sum[i1 + num2.len()] = addition;
                addition = 0;
            }
        }
        let end = sum.iter().rposition(|&item| item != 0).unwrap_or(0);
        let sum = &mut sum[0..end + 1];
        sum.iter_mut().for_each(|item| *item += zero);
        sum.reverse();
        unsafe { String::from_utf8_unchecked(sum.to_vec()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::multiply(String::from("2"), String::from("3")),
            "6"
        );
        assert_eq!(
            Solution::multiply(String::from("123"), String::from("456")),
            "56088"
        );
        assert_eq!(
            Solution::multiply(String::from("9133"), String::from("0")),
            "0"
        );
        assert_eq!(
            Solution::multiply(String::from("9"), String::from("9")),
            "81"
        );
    }
}
