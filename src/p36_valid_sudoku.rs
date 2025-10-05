struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_has = [[false; 9]; 9]; // 第 i 行是否有数字 x
        let mut col_has = [[false; 9]; 9]; // 第 j 列是否有数字 x
        let mut sub_box_has = [[false; 9]; 9];

        for i in 0..board.len() {
            for j in 0..board.len() {
                let c = board[i][j]; // char
                if c == '.' {
                    continue;
                }
                let x: usize = (c as u8 - b'1') as usize; // 1-9 转成 0-8

                if row_has[i][x] || col_has[j][x] || sub_box_has[3 * (i / 3) + j / 3][x] {
                    return false;
                }
                row_has[i][x] = true;
                col_has[j][x] = true;
                sub_box_has[3 * (i / 3) + j / 3][x] = true;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_valid_sudoku(self::to_vec_2d([
                ["5", "3", ".", ".", "7", ".", ".", ".", "."],
                ["6", ".", ".", "1", "9", "5", ".", ".", "."],
                [".", "9", "8", ".", ".", ".", ".", "6", "."],
                ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
                ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
                ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
                [".", "6", ".", ".", ".", ".", "2", "8", "."],
                [".", ".", ".", "4", "1", "9", ".", ".", "5"],
                [".", ".", ".", ".", "8", ".", ".", "7", "9"]
            ])),
            true
        );
        assert_eq!(
            Solution::is_valid_sudoku(self::to_vec_2d([
                ["8", "3", ".", ".", "7", ".", ".", ".", "."],
                ["6", ".", ".", "1", "9", "5", ".", ".", "."],
                [".", "9", "8", ".", ".", ".", ".", "6", "."],
                ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
                ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
                ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
                [".", "6", ".", ".", ".", ".", "2", "8", "."],
                [".", ".", ".", "4", "1", "9", ".", ".", "5"],
                [".", ".", ".", ".", "8", ".", ".", "7", "9"]
            ])),
            false
        );
    }

    fn to_vec_2d(arr: [[&'static str; 9]; 9]) -> Vec<Vec<char>> {
        arr.iter()
            .map(|row| row.map(|col| col.chars().next().unwrap()).to_vec())
            .collect()
    }
}
