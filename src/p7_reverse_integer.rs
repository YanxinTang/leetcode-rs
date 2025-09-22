struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x: i32 = x;
        let mut ret: i32 = 0;

        while x != 0 {
            let digit = x % 10;
            if !(i32::MIN / 10..=i32::MAX / 10).contains(&ret) {
                return 0;
            }
            ret = ret * 10 + digit;
            x /= 10;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(2147483638), 0);
    }
}
