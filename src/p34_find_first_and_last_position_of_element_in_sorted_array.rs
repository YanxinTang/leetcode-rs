struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let (mut ans_l, mut ans_r) = (-1, -1);
        {
            let (mut l, mut r) = (0, nums.len() - 1);
            while l <= r {
                let mid = (l + r) / 2;
                if (mid == 0 || nums[mid - 1] != target) && nums[mid] == target {
                    ans_l = mid as i32;
                    break;
                }
                if nums[mid] < target {
                    l = mid + 1;
                } else {
                    if mid == 0 {
                        break;
                    }
                    r = mid - 1;
                }
            }
        }
        {
            let (mut l, mut r) = (0, nums.len() - 1);
            while l <= r {
                let mid = (l + r) / 2;
                if (mid == nums.len() - 1 || nums[mid + 1] != target) && nums[mid] == target {
                    ans_r = mid as i32;
                    break;
                }
                if nums[mid] <= target {
                    l = mid + 1;
                } else {
                    if mid == 0 {
                        break;
                    }
                    r = mid - 1;
                }
            }
        }

        vec![ans_l, ans_r]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![1], 0), vec![-1, -1]);
        assert_eq!(Solution::search_range(vec![1], 1), vec![0, 0]);
    }
}
