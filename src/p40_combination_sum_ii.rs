struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn back_trace(
            candidates: &[i32],
            target: i32,
            ans: &mut Vec<Vec<i32>>,
            combine: &mut Vec<i32>,
            idx: usize,
        ) {
            if target == 0 {
                ans.push(combine.to_vec());
                return;
            }
            if idx == candidates.len() {
                return;
            }

            if target - candidates[idx] >= 0 {
                // 选择当前数
                combine.push(candidates[idx]);
                // idx + 1: 垂直剪枝，已选过的数不能重复选择
                back_trace(candidates, target - candidates[idx], ans, combine, idx + 1);
                combine.pop();
            }

            // 水平前枝，跳过重复处理过的数
            let mut next_idx = idx + 1;
            while next_idx < candidates.len() && candidates[next_idx] == candidates[idx] {
                next_idx += 1;
            }
            back_trace(candidates, target, ans, combine, next_idx); // 不选当前数
        }

        let mut ans: Vec<Vec<i32>> = vec![];
        let mut combine: Vec<i32> = vec![];
        let mut candidates = candidates;
        candidates.sort();
        back_trace(&candidates, target, &mut ans, &mut combine, 0);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![1, 2, 2], vec![5]]
        );
    }
}
