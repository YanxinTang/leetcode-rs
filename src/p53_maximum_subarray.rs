struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans: i32 = i32::MIN;
        let mut sum: i32 = 0;
        for num in nums {
            sum = sum.max(0) + num;
            ans = ans.max(sum)
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(Solution::max_sub_array(vec![-1]), -1);
        assert_eq!(Solution::max_sub_array(vec![-2, 1]), 1);
        assert_eq!(Solution::max_sub_array(vec![-2, -1]), -1);
        assert_eq!(Solution::max_sub_array(vec![-1, 1, 2, 1]), 4);
    }
}
