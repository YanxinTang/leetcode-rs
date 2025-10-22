struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut ans = -1;
        let (mut l, mut r) = (0, x);
        while l <= r {
            let mid = l + (r - l) / 2;
            let sqrt: i64 = mid as i64 * mid as i64;
            if sqrt <= x as i64 {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }
}
