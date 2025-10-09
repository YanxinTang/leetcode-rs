struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash_map = std::collections::HashMap::<Vec<u8>, Vec<String>>::new();
        for item in strs {
            let mut key = item.as_bytes().to_vec();
            key.sort();
            hash_map.entry(key).or_default().push(item);
        }
        hash_map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::group_anagrams(vec![
                String::from("eat"),
                String::from("tea"),
                String::from("tan"),
                String::from("ate"),
                String::from("nat"),
                String::from("bat")
            ]),
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
        );
    }
}
