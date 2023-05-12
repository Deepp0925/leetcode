pub fn longest_palindrome(s: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
        assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
        assert_eq!(longest_palindrome("a".to_string()), "a".to_string());
        assert_eq!(longest_palindrome("ac".to_string()), "a".to_string());
    }
}
