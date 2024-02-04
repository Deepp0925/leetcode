pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let x = x.to_string();
    let len = x.len();

    for i in 0..len {
        let last = len - 1 - i;
        if i >= last {
            break;
        }

        if x[i..i + 1] != x[last..last + 1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(-101), false);
    }

    #[test]
    fn other_test() {
        println!("{:?}", 3 / 2);
    }
}
