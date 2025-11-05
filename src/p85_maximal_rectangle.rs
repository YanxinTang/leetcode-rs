struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let mut ans: usize = 0;

        // 从上到下，跟上面的层组合起来，求解柱状图中最大矩形
        let n = matrix[0].len();
        let mut heights: Vec<usize> = vec![0; n];

        for row in matrix {
            for (j, item) in row.iter().enumerate() {
                match item {
                    '1' => {
                        heights[j] += 1;
                    }
                    _ => {
                        heights[j] = 0;
                    }
                }
            }
            ans = ans.max(Self::largest_rect(&heights));
        }
        ans as i32
    }

    /// 计算直方图中最大矩形面积（单调栈法）
    fn largest_rect(heights: &[usize]) -> usize {
        let mut ans = 0;
        let mut stack: Vec<usize> = Vec::with_capacity(heights.len());
        for (i, &height) in heights.iter().enumerate() {
            // 一直出栈，直到比新栈顶大，并计算面积
            while let Some(&new_top) = stack.last()
                && height < heights[new_top]
            {
                stack.pop();
                let width = if let Some(&new_top) = stack.last() {
                    i - new_top - 1
                } else {
                    i
                };
                ans = ans.max(width * heights[new_top]);
            }
            stack.push(i);
        }

        let n = heights.len();
        while let Some(&new_top) = stack.last() {
            stack.pop();
            let width = if let Some(&new_top) = stack.last() {
                n - new_top - 1
            } else {
                n
            };
            ans = ans.max(width * heights[new_top]);
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
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            6
        );
    }
}
