struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn back_trace(
            nums: &[i32],
            ans: &mut Vec<Vec<i32>>,
            candidates: &mut Vec<i32>,
            access_flag: &mut Vec<bool>,
        ) {
            if candidates.len() == nums.len() {
                ans.push(candidates.to_vec());
                return;
            }

            for (i, &item) in nums.iter().enumerate() {
                if access_flag[i] {
                    continue;
                }
                if i > 0 && nums[i] == nums[i - 1] && !access_flag[i - 1] {
                    continue;
                }
                access_flag[i] = true;
                candidates.push(item);
                back_trace(nums, ans, candidates, access_flag);
                candidates.pop();
                access_flag[i] = false;
            }
        }

        let mut nums = nums;
        nums.sort();
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut candidates: Vec<i32> = vec![];
        let mut access_flag = vec![false; nums.len()];
        back_trace(&nums, &mut ans, &mut candidates, &mut access_flag);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
        assert_eq!(
            Solution::permute_unique(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
