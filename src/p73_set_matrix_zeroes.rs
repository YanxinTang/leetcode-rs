struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut row_flag = vec![false; m];
        let mut col_flag = vec![false; n];
        matrix.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &col)| {
                if col == 0 {
                    row_flag[i] = true;
                    col_flag[j] = true;
                }
            });
        });
        matrix.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, col)| {
                if row_flag[i] || col_flag[j] {
                    *col = 0;
                }
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        {
            let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
            Solution::set_zeroes(&mut matrix);
            assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
        }
        {
            let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
            Solution::set_zeroes(&mut matrix);
            assert_eq!(
                matrix,
                vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
            );
        }
    }
}
