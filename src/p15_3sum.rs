struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut ans: Vec<Vec<i32>> = vec![];
        for i in 0..nums.len() - 2 {
            let num1 = nums[i];
            if i > 0 && num1 == nums[i - 1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let num2 = nums[j];
                let num3 = nums[k];

                if num1 + num2 + num3 == 0 {
                    j += 1;
                    k -= 1;
                    ans.push(vec![num1, num2, num3]);

                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                } else if num1 + num2 + num3 > 0 {
                    k -= 1; // 使和合减小
                } else {
                    j += 1; // 使和增大
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
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(
            Solution::three_sum(vec![-100, -70, -60, 110, 120, 130, 160]),
            vec![vec![-100, -60, 160], vec![-70, -60, 130]]
        );

        assert_eq!(
            Solution::three_sum(vec![0, 1, 1]),
            Vec::<Vec<i32>>::with_capacity(0)
        );
        assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]]);
    }
}
