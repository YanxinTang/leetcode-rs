use std::vec;

struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut ans: Vec<Vec<i32>> = vec![];

        for interval in intervals {
            if let Some(top) = ans.last_mut()
                && top[1] >= interval[0]
            {
                top[1] = top[1].max(interval[1]);
                continue;
            }
            ans.push(interval);
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
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
        assert_eq!(
            Solution::merge(vec![vec![4, 7], vec![1, 4]]),
            vec![vec![1, 7]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![2, 3]]),
            vec![vec![1, 4]]
        );
    }
}
