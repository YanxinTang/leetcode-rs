struct Solution;

impl Solution {
    /// 先对数组排序，再使用双指针法遍历所有可能解
    /// 和比目标小的情况，左指针向右移动，使和增大
    /// 和比目标大的情况，右指针向左移动，使和减小
    /// 指针移动后，跳过重复的值
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort(); // 升序

        let d = |a: i32| -> i32 { i32::abs(target - a) }; // 计算给定数与 target 的距离

        let mut ans: i32 = nums[0] + nums[1] + nums[2];
        let mut ans_d = d(ans);
        for i in 0..nums.len() - 2 {
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == target {
                    return target;
                } else {
                    if sum > target {
                        r -= 1; // 移动右指针，使和减小

                        // 对于连续重复的右指针值，如果与上一个相等，直接跳过
                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1
                        }
                    } else {
                        l += 1; // 和比目标小，移动左指针，使和增大

                        // 对于连续重复的左指针值，如果与上一个相等，直接跳过
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1
                        }
                    }
                    let new_ans_d = d(sum);
                    if new_ans_d < ans_d {
                        ans = sum;
                        ans_d = new_ans_d;
                    }
                }
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
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0);
    }
}
