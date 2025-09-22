struct Solution;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows: usize = num_rows as usize;
        let len = s.chars().count();
        if len <= num_rows || num_rows <= 1 {
            return s;
        }

        let s = s.chars();
        let mut ret = vec![Vec::<char>::with_capacity(len); num_rows];

        let mut i = 0;
        let mut factor: i8 = 1;
        for char in s {
            ret[i].push(char);
            if factor > 0 {
                i += 1
            } else {
                i -= 1
            }

            if i == 0 {
                factor = 1;
            } else if i == num_rows - 1 {
                factor = -1;
            }
        }

        ret.iter().flatten().collect()
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(
            Solution::convert(String::from("PAYPALISHIRING"), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(Solution::convert(String::from("A"), 1), "A");
        assert_eq!(Solution::convert(String::from("AB"), 1), "AB");
    }
}
