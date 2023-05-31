pub fn longest_palindrome(s: String) -> String {
    let len = s.len();
    if len == 1 {
        return s;
    }

    let mut longest = String::new();
    for i in 0..len {
        let mut left = i;
        let mut right = i;

        while left > 0 && s.chars().nth(left - 1) == s.chars().nth(i) {
            left -= 1;
        }

        while right < len - 1 && s.chars().nth(right + 1) == s.chars().nth(i) {
            right += 1;
        }

        while left > 0 && right < len - 1 && s.chars().nth(left - 1) == s.chars().nth(right + 1) {
            left -= 1;
            right += 1;
        }

        let curr = &s[left..=right];
        if curr.len() > longest.len() {
            longest = curr.to_string();
        }
    }

    longest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome("babad".to_string()), "aba".to_string());
        assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
        assert_eq!(longest_palindrome("a".to_string()), "a".to_string());
        assert_eq!(longest_palindrome("ac".to_string()), "a".to_string());
        let s = "z".repeat(100);
        assert_eq!(longest_palindrome(s.clone()), s);
    }

    #[test]
    fn max_test() {
        println!("{}", std::cmp::max(1, 2));
        println!("{}", std::cmp::max("A", "B"));
    }
}
