struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(
            board: &[Vec<char>],
            chars: &[char],
            visited: &mut [Vec<bool>],
            i: usize,
            j: usize,
        ) -> bool {
            if board[i][j] != chars[0] {
                return false;
            }
            if chars.len() == 1 {
                return true;
            }
            let m = board.len();
            let n = board[0].len();

            visited[i][j] = true;
            // 从上右下左四个方向寻找 word[1..] 子串
            if i > 0 && !visited[i - 1][j] && dfs(board, &chars[1..], visited, i - 1, j) {
                return true;
            }
            if j < n - 1 && !visited[i][j + 1] && dfs(board, &chars[1..], visited, i, j + 1) {
                return true;
            }
            if i < m - 1 && !visited[i + 1][j] && dfs(board, &chars[1..], visited, i + 1, j) {
                return true;
            }
            if j > 0 && !visited[i][j - 1] && dfs(board, &chars[1..], visited, i, j - 1) {
                return true;
            }
            visited[i][j] = false;
            false
        }
        let m = board.len();
        let n = board[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];
        let chars: Vec<char> = word.chars().collect();
        for (i, row) in board.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if dfs(&board, &chars, &mut visited, i, j) {
                    return true;
                }
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
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                String::from("ABCCED")
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                String::from("SEE")
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                String::from("ABCB")
            ),
            false
        );
    }
}
