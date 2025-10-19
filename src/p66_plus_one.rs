struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;

        let mut carry = 1;
        for d in digits.iter_mut().rev() {
            *d += carry;
            carry = *d / 10;
            *d %= 10;
            if carry == 0 {
                break;
            }
        }

        if carry > 0 {
            digits.insert(0, carry);
        }

        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
        assert_eq!(Solution::plus_one(vec![9, 9]), vec![1, 0, 0]);
    }
}
