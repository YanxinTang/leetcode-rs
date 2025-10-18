struct Solution;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        fn back_trace(n: i32, k: &mut i32, path: &mut Vec<i32>, used: &mut Vec<bool>) -> bool {
            if path.len() as i32 == n {
                return true;
            }
            let factorial = (1..=n - 1 - path.len() as i32)
                .reduce(|fac, e| fac * e)
                .unwrap_or(1);
            for i in 1..=n {
                if used[i as usize] {
                    continue;
                }
                if factorial < *k {
                    *k -= factorial;
                    continue;
                }
                path.push(i);
                used[i as usize] = true;
                if back_trace(n, k, path, used) {
                    return true;
                };
            }
            false
        }

        let mut path = vec![];
        let mut used = vec![false; n as usize + 1];
        let mut k = k;
        back_trace(n, &mut k, &mut path, &mut used);
        path.iter().map(i32::to_string).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::get_permutation(3, 3), "213");
        assert_eq!(Solution::get_permutation(4, 9), "2314");
        assert_eq!(Solution::get_permutation(3, 1), "123");
    }
}
