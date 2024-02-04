pub fn longest_palindrome(s: String) -> String {
    let len = s.len();
    if len == 1 {
        return s;
    }

    fn expand_around_center(s: &str, l: usize, mut r: usize) -> usize {
        let mut l = l as i32;
        while l >= 0 && r < s.len() && s[l as usize..l as usize + 1] == s[r..r + 1] {
            l -= 1;
            r += 1;
        }

        (r as i32 - l - 1) as usize
    }

    let mut start = 0;
    let mut max_len = 0;

    for i in 0..len {
        // check for odd lenght palindromes
        let l1 = expand_around_center(&s, i, i);
        let l2 = expand_around_center(&s, i, i + 1);

        let max = l1.max(l2);
        if max > max_len {
            max_len = max;
            start = i - ((max - 1) / 2);
        }
    }
    s[start..start + max_len].to_string()
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
        let s = "z".repeat(4);
        assert_eq!(longest_palindrome(s.clone()), s);
        let s = "c".repeat(3);
        assert_eq!(longest_palindrome(s.clone()), s);
    }
}
