struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if nums.len() < 4 {
            return Vec::<Vec<i32>>::with_capacity(0);
        }

        let mut nums = nums;
        nums.sort();

        let three_sum = |nums: &[i32], target: i64, r_init: usize| -> (Vec<Vec<i32>>, usize) {
            let mut ans: Vec<Vec<i32>> = vec![];

            let mut r = r_init; // 最右指针
            for i in 0..nums.len() - 2 {
                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }
                let mut l = i + 1;
                r = nums.len() - 1;

                while l < r {
                    let sum = nums[i] as i64 + nums[l] as i64 + nums[r] as i64;
                    if sum == target {
                        ans.push(vec![nums[i], nums[l], nums[r]]);
                        l += 1;
                        while l < r && nums[l] == nums[l - 1] {
                            l += 1
                        }
                        r -= 1;
                        while l < r && nums[r] == nums[r + 1] {
                            r -= 1
                        }
                    } else if sum < target {
                        l += 1;
                    } else {
                        r -= 1;
                    }
                }
            }
            (ans, r)
        };

        let mut ans: Vec<Vec<i32>> = vec![];

        // 三数之和的每轮遍历过程中，和总是越来越大
        // 下一轮遍历，和会更大一点，所以右指针不可能比上一轮更右，因为这会导致和更大，
        // 每次遍历右指针从上次最后的右指针位置开始
        let mut last_r: usize = nums.len() - 4;
        let target = target as i64;
        for i in 0..nums.len() - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            // 检查是否会溢出
            let (mut three_sum_ans, r) = three_sum(&nums[i + 1..], target - nums[i] as i64, last_r);
            last_r = r;
            three_sum_ans
                .iter_mut()
                .for_each(|item| item.insert(0, nums[i]));
            ans.append(&mut three_sum_ans);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::four_sum(
                vec![1000000000, 1000000000, 1000000000, 1000000000],
                -294967296
            ),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            Solution::four_sum(vec![-2, -1, -1, 1, 1, 2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-1, -1, 1, 1]]
        );
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![vec![2, 2, 2, 2]]
        );
        assert_eq!(Solution::four_sum(vec![0], 0), Vec::<Vec<i32>>::new());
    }
}
