struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        // 应该用栈来解这道题
        let mut stack: Vec<String> = vec![];

        for node in path.split("/") {
            match node {
                "" | "." => {}
                ".." => {
                    stack.pop();
                }
                _ => stack.push(node.to_string()),
            }
        }
        format!("/{}", stack.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::simplify_path(String::from("/home/")), "/home");
        assert_eq!(
            Solution::simplify_path(String::from("/home//foo/")),
            "/home/foo"
        );
        assert_eq!(
            Solution::simplify_path(String::from("/home/user/Documents/../Pictures")),
            "/home/user/Pictures"
        );
        assert_eq!(Solution::simplify_path(String::from("/../")), "/");
        assert_eq!(
            Solution::simplify_path(String::from("/.../a/../b/c/../d/./")),
            "/.../b/d"
        );
    }
}
