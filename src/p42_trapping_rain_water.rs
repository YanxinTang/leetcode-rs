struct Solution;

impl Solution {
    // 一、动态规划
    // left_max[i] = max(left_max[i-1], height[i]): i 及其左侧最高的柱子
    // right_max[i] = max(right_max[i+1], height[i]): i 及其右侧最高的柱子
    // i 处所能接到的雨水：min(left_max[i], right_max[i]) - height[i]
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_max: Vec<i32> = vec![0; height.len()];
        let mut right_max: Vec<i32> = vec![0; height.len()];
        for (i, &item) in height.iter().enumerate() {
            if i == 0 {
                left_max[i] = item;
            } else {
                left_max[i] = item.max(left_max[i - 1]);
            }
        }
        for (i, &item) in height.iter().enumerate().rev() {
            if i == height.len() - 1 {
                right_max[i] = item;
            } else {
                right_max[i] = item.max(right_max[i + 1]);
            }
        }
        let mut ans: i32 = 0;
        for (i, &item) in height.iter().enumerate() {
            ans += i32::min(left_max[i], right_max[i]) - item;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
