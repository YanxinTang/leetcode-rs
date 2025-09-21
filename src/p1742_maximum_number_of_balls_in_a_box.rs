use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in low_limit..=high_limit {
            let mut num = i;
            let mut sum = 0;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }
            map.entry(sum).and_modify(|count| *count += 1).or_insert(1);
        }
        map.values().max().cloned().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::count_balls(1, 10), 2);
        assert_eq!(Solution::count_balls(5, 15), 2);
        assert_eq!(Solution::count_balls(19, 28), 2);
    }
}
