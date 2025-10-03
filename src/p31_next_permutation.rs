struct Solution;

impl Solution {
    // 从右往左找升序的截止位置 i, a[i + 1..] 为降序排列
    // 再从右往左找一个数 a[j] 大于 a[i]，交换 a[i] 和 a[j]
    // 因为 a[j] > a[i], a[i] > a[j + 1]，所以交换之后，a[i + 1..] 仍然为降序
    // 降序是最大序列，我们反转 a[i+1..]，将其变为升序，也就是变成最小序列
    pub fn next_permutation(nums: &mut [i32]) {
        let mut i: isize = nums.len() as isize - 2; // 从倒数第二个开始比较
        while i >= 0 && nums[i as usize] >= nums[i as usize + 1] {
            i -= 1;
        }

        let usize_i = i as usize;
        if i >= 0 {
            let mut j = nums.len() - 1;
            // 走到这个分支，必定有一个 j 比 i 大
            while nums[j] <= nums[usize_i] {
                j -= 1;
            }
            (nums[usize_i], nums[j]) = (nums[j], nums[usize_i]);
        }

        nums[(i + 1) as usize..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        {
            let mut input: Vec<i32> = vec![1, 2];
            Solution::next_permutation(&mut input);
            assert_eq!(input, vec![2, 1]);
        }
        {
            let mut input: Vec<i32> = vec![1, 2, 3];
            Solution::next_permutation(&mut input);
            assert_eq!(input, vec![1, 3, 2]);
        }
        {
            let mut input: Vec<i32> = vec![3, 2, 1];
            Solution::next_permutation(&mut input);
            assert_eq!(input, vec![1, 2, 3]);
        }
        {
            let mut input: Vec<i32> = vec![1, 1, 5];
            Solution::next_permutation(&mut input);
            assert_eq!(input, vec![1, 5, 1]);
        }
    }
}
