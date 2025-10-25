struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // p0: 下一个 0 放的位置
        // p1：下一个 1 放的位置
        let (mut p0, mut p1) = (0, 0);
        for i in 0..nums.len() {
            let num = nums[i];
            nums[i] = 2;
            if num <= 1 {
                nums[p1] = 1;
                p1 += 1;
            }
            if num == 0 {
                nums[p0] = 0;
                p0 += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        {
            let mut nums = vec![2, 0, 2, 1, 1, 0];
            Solution::sort_colors(&mut nums);
            assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
        }
        {
            let mut nums = vec![2, 0, 1];
            Solution::sort_colors(&mut nums);
            assert_eq!(nums, vec![0, 1, 2]);
        }
        {
            let mut nums = vec![0];
            Solution::sort_colors(&mut nums);
            assert_eq!(nums, vec![0]);
        }
    }
}
