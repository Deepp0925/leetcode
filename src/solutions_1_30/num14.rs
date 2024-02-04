pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut ans = String::new();

    let chars = strs[0].chars().peekable();

    for (i, char) in chars.enumerate() {
        for s in &strs {
            if let Some(c) = s.chars().nth(i) {
                if c != char {
                    return ans;
                }
            } else {
                return ans;
            }
        }

        ans.push(char);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            String::from("fl")
        );
        assert_eq!(
            longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ]),
            String::from("")
        );
        assert_eq!(
            longest_common_prefix(vec![String::from("a")]),
            String::from("a")
        );

        assert_eq!(
            longest_common_prefix(vec![
                String::from("flowering"),
                String::from("flower"),
                String::from("flowers"),
            ]),
            String::from("flower")
        );
    }
}
