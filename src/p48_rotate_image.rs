struct Solution;

impl Solution {
    // 观察之后发现
    // 第一行 [i, n-1) 中的每个元素，与最后一列，最后一行，第一列对应的位置交换，即可完成旋转
    #[allow(clippy::ptr_arg)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for k in 0..n / 2 {
            let end = matrix.len() - 1 - k;

            for i in 0..n - 1 - 2 * k {
                // 与第最后一列对应位置交换
                (matrix[k][k + i], matrix[k + i][end]) = (matrix[k + i][end], matrix[k][k + i]);
                // 与第最后一行对应位置交换
                (matrix[k][k + i], matrix[end][end - i]) = (matrix[end][end - i], matrix[k][k + i]);
                // 与第一列对应位置交换
                (matrix[k][k + i], matrix[end - i][k]) = (matrix[end - i][k], matrix[k][k + i]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        {
            let mut matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            Solution::rotate(&mut matrix);
            assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
        }
        {
            let mut matrix: Vec<Vec<i32>> = vec![
                vec![5, 1, 9, 11],
                vec![2, 4, 8, 10],
                vec![13, 3, 6, 7],
                vec![15, 14, 12, 16],
            ];
            Solution::rotate(&mut matrix);
            assert_eq!(
                matrix,
                vec![
                    vec![15, 13, 2, 5],
                    vec![14, 3, 4, 1],
                    vec![12, 6, 8, 9],
                    vec![16, 7, 10, 11]
                ]
            );
        }
    }
}
