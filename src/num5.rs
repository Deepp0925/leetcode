pub fn longest_palindrome(s: String) -> String {
    let len = s.len();
    if len < 2 {
        return s;
    }

    let mut dp = vec![vec![false; len]; len];
    let mut start = 0;
    let mut max_len = 1;

    // Single characters are palindromes
    for i in 0..len {
        dp[i][i] = true;
    }

    // Check for palindromes of length 2
    for i in 0..len - 1 {
        let j = i + 1;
        if s.chars().nth(i) == s.chars().nth(j) {
            dp[i][j] = true;
            start = i;
            max_len = 2;
        }
    }

    // Check for palindromes of length > 2
    for k in 3..=len {
        for i in 0..len - k + 1 {
            let j = i + k - 1;
            if dp[i + 1][j - 1] && s.chars().nth(i) == s.chars().nth(j) {
                dp[i][j] = true;
                start = i;
                max_len = k;
            }
        }
    }

    (&s[start..start + max_len]).to_string()
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
