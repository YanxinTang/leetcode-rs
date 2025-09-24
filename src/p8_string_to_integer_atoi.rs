struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = s.trim_ascii_start();
        let mut s = s.chars().peekable();
        while let Some(' ') = s.peek() {
            s.next();
        }

        let sign: i32 = match s.peek() {
            Some('-') => {
                s.next();
                -1
            }
            Some('+') => {
                s.next();
                1
            }
            _ => 1,
        };

        let mut ret: i32 = 0;
        let char_0: i32 = '0' as i32;
        const I32_MIN_DIV_10: i32 = i32::MIN / 10;
        const I32_MIN_REMAINDER_10: i32 = i32::MIN % 10;
        const I32_MAX_DIV_10: i32 = i32::MAX / 10;
        const I32_MAX_REMAINDER_10: i32 = i32::MAX % 10;
        while let Some(&c) = s.peek() {
            if c.is_ascii_digit() {
                let digit: i32 = c as i32 - char_0;
                if ret > I32_MAX_DIV_10 || (ret == I32_MAX_DIV_10 && digit > I32_MAX_REMAINDER_10) {
                    return if sign == 1 { i32::MAX } else { i32::MIN };
                }
                ret = ret * 10 + digit;
            } else {
                break;
            }
            s.next();
        }
        ret * sign
    }

    pub fn my_atoi1(s: String) -> i32 {
        let s = s.trim_ascii_start();
        let s: Vec<char> = s.chars().collect();
        let mut s: &[char] = &s;

        match s.first() {
            Some(&f) => {
                let mut factor: i32 = 1;
                if f == '-' || f == '+' {
                    s = &s[1..];
                    if f == '-' {
                        factor = -1;
                    }
                }

                let mut ret: i32 = 0;
                let char_0: i32 = '0' as i32;
                const I32_MIN_DIV_10: i32 = i32::MIN / 10;
                const I32_MIN_REMAINDER_10: i32 = i32::MIN % 10;
                const I32_MAX_DIV_10: i32 = i32::MAX / 10;
                const I32_MAX_REMAINDER_10: i32 = i32::MAX % 10;
                for &c in s {
                    if c.is_ascii_digit() {
                        let digit: i32 = factor * (c as i32 - char_0);

                        if ret < I32_MIN_DIV_10
                            || (ret == I32_MIN_DIV_10 && digit < I32_MIN_REMAINDER_10)
                        {
                            return i32::MIN;
                        }
                        if ret > I32_MAX_DIV_10
                            || (ret == I32_MAX_DIV_10 && digit > I32_MAX_REMAINDER_10)
                        {
                            return i32::MAX;
                        }
                        ret = ret * 10 + digit;
                    } else {
                        break;
                    }
                }

                ret
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::my_atoi(String::from("42")), 42);
        assert_eq!(Solution::my_atoi(String::from("-042")), -42);
        assert_eq!(Solution::my_atoi(String::from("1337c0d3")), 1337);
        assert_eq!(Solution::my_atoi(String::from("0-1")), 0);
        assert_eq!(Solution::my_atoi(String::from("words and 987")), 0);
        assert_eq!(Solution::my_atoi(String::from("   -042")), -42);
        assert_eq!(Solution::my_atoi(String::from("2147483648")), 2147483647);
        assert_eq!(Solution::my_atoi(String::from("2147483647")), 2147483647);
        assert_eq!(Solution::my_atoi(String::from("2147483648")), 2147483647);
        assert_eq!(Solution::my_atoi(String::from("-2147483648")), -2147483648);
        assert_eq!(Solution::my_atoi(String::from("-2147483649")), -2147483648);
        assert_eq!(Solution::my_atoi(String::from("-91283472332")), -2147483648);
    }
}
