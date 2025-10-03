struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        let negative = (dividend >= 0) ^ (divisor >= 0);
        let mut dividend = if dividend > 0 { -dividend } else { dividend };
        let divisor = if divisor > 0 { -divisor } else { divisor };
        let mut ans = 0;
        while dividend <= divisor {
            let (mut tmp, mut multiple) = (divisor, 1);
            while dividend <= tmp << 1 && tmp << 1 < 0 {
                tmp <<= 1;
                multiple <<= 1;
            }

            dividend -= tmp;
            ans += multiple;
        }
        if negative { -ans } else { ans }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
        assert_eq!(Solution::divide(-2147483648, -2147483648), 1);
    }
}
