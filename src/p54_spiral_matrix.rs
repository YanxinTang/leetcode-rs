struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans: Vec<i32> = Vec::with_capacity(m * n);
        let mut ring: usize = 0; // 第 n 圈
        let (mut i, mut j) = (0, 0);
        enum Direction {
            Up,
            Right,
            Down,
            Left,
        }
        let mut direction: Direction = Direction::Right;

        for _ in 0..m * n {
            ans.push(matrix[i][j]);

            match direction {
                Direction::Right => {
                    if j == n - ring - 1 {
                        direction = Direction::Down;
                        i += 1
                    } else {
                        j += 1
                    }
                }
                Direction::Down => {
                    if i == m - ring - 1 {
                        direction = Direction::Left;
                        j -= 1;
                    } else {
                        i += 1
                    }
                }
                Direction::Left => {
                    if j == ring {
                        direction = Direction::Up;
                        i -= 1;
                        ring += 1;
                    } else {
                        j -= 1
                    }
                }
                Direction::Up => {
                    if i == ring {
                        direction = Direction::Right;
                        j += 1;
                    } else {
                        i -= 1
                    }
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
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
