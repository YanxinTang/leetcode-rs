struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let mid = l + (r - l) / 2;
            if target == nums[mid] {
                return true;
            }
            if nums[l] == nums[mid] {
                l += 1;
            } else if nums[l] < nums[mid] {
                if nums[l] <= target && target < nums[mid] {
                    r = mid;
                } else {
                    l = mid + 1;
                }
            } else if nums[mid] < target && target <= nums[nums.len() - 1] {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::search(vec![3, 1], 3), true);
        assert_eq!(
            Solution::search(
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
                2
            ),
            true
        );
        assert_eq!(Solution::search(vec![1], 1), true);
        assert_eq!(Solution::search(vec![1], 0), false);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    }
}
