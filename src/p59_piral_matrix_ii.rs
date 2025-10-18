struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let len = n * n;
        let mut ans: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];
        let (mut m, mut n) = (n, n); // m 行 n 列
        let mut pos: (i32, i32) = (0, -1);
        let mut dir_idx = 0;
        let mut i = 1;
        while i <= len {
            let dir = DIRS[dir_idx];
            for _ in 0..n {
                pos.0 += dir.0;
                pos.1 += dir.1;
                ans[pos.0 as usize][pos.1 as usize] = i;
                i += 1;
            }
            (m, n) = (n, m - 1);
            dir_idx = (dir_idx + 1) % 4;
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
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }
}
