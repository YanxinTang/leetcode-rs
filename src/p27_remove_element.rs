struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
        let (mut slow, mut fast) = (0, 0);
        while fast < nums.len() {
            if nums[fast] != val {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }
        slow as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        {
            let mut nums = vec![3, 2, 2, 3];
            assert_eq!(Solution::remove_element(&mut nums, 3), 2);
            assert_eq!(nums[0..2], vec![2, 2]);
        }
        {
            let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
            assert_eq!(Solution::remove_element(&mut nums, 2), 5);
            assert_eq!(nums[0..5], vec![0, 1, 3, 0, 4]);
        }
    }
}
