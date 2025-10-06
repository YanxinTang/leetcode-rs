struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut nums = nums;
        nums.iter_mut().for_each(|item| {
            if *item <= 0 || *item > n {
                *item = n + 1;
            }
        });
        for i in 0..nums.len() {
            if nums[i].abs() <= n {
                let idx = (nums[i].abs() - 1) as usize;
                if nums[idx] > 0 {
                    nums[idx] = -nums[idx];
                }
            }
        }
        for i in 0..nums.len() {
            if nums[i] > 0 {
                return (i + 1) as i32;
            }
        }
        n + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
        assert_eq!(Solution::first_missing_positive(vec![1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
    }
}
