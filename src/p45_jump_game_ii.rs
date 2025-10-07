struct Solution;

#[allow(clippy::needless_range_loop)]
impl Solution {
    // 从后往前找
    // 假如 n[i] 是离 n-1 最远的一个可能的结果，那跳转到 i 后边的位置，最优解也跟跳转到 n[i] 相等
    // 所以首先把目标位置设为 n-1，找一个最靠左的能跳到 n-1 的最优位置 k
    // 如法炮制，找最靠左的跳到 k 的位置
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut jump_to = nums.len() - 1;
        let mut ans = 0;
        while jump_to > 0 {
            for i in 0..jump_to {
                if nums[i] as usize + i >= jump_to {
                    jump_to = i;
                    break;
                }
            }
            ans += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::jump(vec![2, 1]), 1);
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
