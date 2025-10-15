struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut new_interval = new_interval;

        let mut i = 0;
        while i < intervals.len() && intervals[i][1] < new_interval[0] {
            ans.push(intervals[i].clone());
            i += 1;
        }
        while i < intervals.len() && (intervals[i][0] <= new_interval[1]) {
            new_interval[0] = intervals[i][0].min(new_interval[0]);
            new_interval[1] = intervals[i][1].max(new_interval[1]);
            i += 1;
        }
        ans.push(new_interval);
        while i < intervals.len() {
            ans.push(intervals[i].clone());
            i += 1;
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
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }
}
