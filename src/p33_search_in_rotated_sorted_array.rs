struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l <= r {
            let mid = (l + r) / 2;
            if nums[mid] == target {
                return mid as i32;
            }

            if nums[l] <= nums[mid] {
                // 如果 [l, mid) 单调递增区间
                if nums[l] <= target && target < nums[mid] {
                    r = mid - 1;
                } else {
                    l = mid + 1;
                }
            } else {
                // 如果 [mid, r) 单调递增区别
                if nums[mid] < target && target <= nums[r] {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::search(vec![3, 1], 1), 1);
        assert_eq!(Solution::search(vec![1, 3], 3), 1);
        assert_eq!(Solution::search(vec![1, 3], 1), 0);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
    }
}
