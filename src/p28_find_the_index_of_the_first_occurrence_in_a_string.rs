struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }

        let haystack: Vec<char> = haystack.chars().collect();
        let needle: Vec<char> = needle.chars().collect();
        'outer: for i in 0..haystack.len() - needle.len() + 1 {
            for j in 0..needle.len() {
                if haystack[i + j] != needle[j] {
                    continue 'outer;
                }
            }
            return i as i32;
        }
        -1
    }

    // TODO: KMP 算法实现
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::str_str(String::from("a"), String::from("a")), 0);
        assert_eq!(Solution::str_str(String::from("abc"), String::from("c")), 2);
        assert_eq!(
            Solution::str_str(String::from("sadbutsad"), String::from("sad")),
            0
        );
        assert_eq!(
            Solution::str_str(String::from("leetcode"), String::from("leeto")),
            -1
        );
        assert_eq!(
            Solution::str_str(String::from("aleeleetcode"), String::from("leet")),
            4
        );
    }
}
