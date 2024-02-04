pub fn is_match(s: String, p: String) -> bool {
    let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
    dp[0][0] = true;
    for i in 0..=s.len() {
        for j in 1..=p.len() {
            if p.chars().nth(j - 1).unwrap() == '*' {
                dp[i][j] = dp[i][j - 2]
                    || (i > 0
                        && (s.chars().nth(i - 1).unwrap() == p.chars().nth(j - 2).unwrap()
                            || p.chars().nth(j - 2).unwrap() == '.')
                        && dp[i - 1][j]);
            } else {
                dp[i][j] = i > 0
                    && dp[i - 1][j - 1]
                    && (s.chars().nth(i - 1).unwrap() == p.chars().nth(j - 1).unwrap()
                        || p.chars().nth(j - 1).unwrap() == '.');
            }
        }
    }
    dp[s.len()][p.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert_eq!(is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(is_match("aa".to_string(), "a*".to_string()), true);
        assert_eq!(is_match("ab".to_string(), ".*".to_string()), true);
        assert_eq!(is_match("aab".to_string(), "c*a*b".to_string()), true);
        assert_eq!(
            is_match("mississippi".to_string(), "mis*is*p*.".to_string()),
            false
        );
    }
}
