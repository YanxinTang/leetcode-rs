struct Solution;

impl Solution {
    // 从最后一个下标开始找最靠左能跳到的位置
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut r = nums.len() - 1;
        let mut l = r;
        while l > 0 {
            l -= 1; // 向左走一步
            if nums[l] >= (r - l) as i32 {
                // 如果 l 能跳到 r，那跳到 l 就相当于能跳到终点
                r = l
            }
        }
        r == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
