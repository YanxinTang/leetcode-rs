use std::vec;

struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        // 00 01 11 10
        // 000 001 011 111 110 100 101 010
        //     0           1
        //   0    1      0    1
        // 0  1  0  1  0  1  0  1
        // 000 001 011 010 110 111 101 100

        let mut ans: Vec<i32> = vec![0, 1];
        for _ in 1..n {
            let mut x = 0b0;
            ans = ans
                .iter()
                .flat_map(|item| {
                    let ret: Vec<i32> = vec![item << 1 | x, item << 1 | (0b1 ^ x)];
                    x ^= 0b1;
                    ret
                })
                .collect();
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::gray_code(3), vec![0, 1, 3, 2, 6, 7, 5, 4]);
        assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
    }
}
