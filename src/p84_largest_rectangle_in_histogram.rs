struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::with_capacity(heights.len());
        let mut ans: i32 = 0;
        for (i, &height) in heights.iter().enumerate() {
            if stack.is_empty() {
                stack.push(i);
                continue;
            }
            let top = stack.last().unwrap();
            if height >= heights[*top] {
                stack.push(i);
            } else {
                // 出栈，计算栈顶能扩散的最大面积
                while let Some(&top) = stack.last()
                    && heights[top] >= height
                {
                    stack.pop();
                    let width = if let Some(&latest_top) = stack.last() {
                        i - latest_top - 1
                    } else {
                        i
                    };
                    ans = ans.max((width) as i32 * heights[top])
                }
                // 计算完成后，把当前元素入栈
                stack.push(i);
            }
        }

        // 计算栈里面剩余元素可勾勒面积
        while let Some(top) = stack.pop() {
            // 相当于把 height.len() 当成最后一个入栈元素，但它的值是 0，所以所有元素要出栈
            let width = if let Some(&latest_top) = stack.last() {
                heights.len() - latest_top - 1
            } else {
                heights.len()
            };
            ans = ans.max((width) as i32 * heights[top])
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::largest_rectangle_area(vec![1]), 1);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    }
}
