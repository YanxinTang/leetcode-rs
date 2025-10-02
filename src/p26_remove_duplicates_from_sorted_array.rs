struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        // 快慢指针，慢指针指向删除后切片最后一个元素，快指针指向最后一个被交换的元素
        // 慢指针一次迭代前进一步
        // 快指针一次迭代前进到与前一个元素不相等的位置
        let (mut slow, mut fast) = (1, 1);

        while fast < nums.len() {
            if nums[fast] != nums[fast - 1] {
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
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 2]), 2);
        assert_eq!(
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
            5
        );
    }
}
