struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut p, mut q) = (0, height.len() - 1);
        let mut max = 0;
        while p < q {
            if height[p] > height[q] {
                max = std::cmp::max(max, (q - p) as i32 * height[q]);
                q -= 1;
            } else {
                max = std::cmp::max(max, (q - p) as i32 * height[p]);
                p += 1;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
