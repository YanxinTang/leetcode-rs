struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let len = m * n;

        let (mut l, mut r) = (0, len);
        while l < r {
            let mid = l + (r - l) / 2;
            let i = mid / n; // 第 i 行
            let j = mid % n; // 第 j 列
            if matrix[i][j] == target {
                return true;
            }
            if target < matrix[i][j] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            ),
            false
        );
        assert_eq!(Solution::search_matrix(vec![vec![1]], 0), false);
        assert_eq!(Solution::search_matrix(vec![vec![1]], 1), true);
        assert_eq!(Solution::search_matrix(vec![vec![1, 1]], 0), false);
    }
}
