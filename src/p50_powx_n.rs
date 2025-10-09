struct Solution;

impl Solution {
    // 假设求解 x^n
    // 把 n 二进制展开：n = ;
    // x^n = x^(b0*2^0 + b1*2^1 + ... + bn*2^n)
    //     = x^(b0*2^0) * x^(b1*2^1) * ... * x^(bn*2^n)
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = if n > 0 { x } else { 1.0 / x };
        let mut n = n.unsigned_abs();
        let mut ans = 1_f64;
        while n > 0 {
            if n & 0b1 > 0 {
                ans *= x;
            }
            x *= x;
            n >>= 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
        assert_eq!(Solution::my_pow(2.1, 3), 9.261000000000001);
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);
    }
}
