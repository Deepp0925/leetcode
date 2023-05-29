pub fn my_atoi(s: String) -> i32 {
    // trims the string
    let mut s = s.trim().as_bytes();

    const MINUS: u8 = 45;
    const PLUS: u8 = 43;
    const ZERO: u8 = 48;
    const NINE: u8 = 57;

    let mut new_str = String::with_capacity(s.len());

    // check if the first char is a sign
    if s.len() > 0 && (s[0] == MINUS || s[0] == PLUS) {
        if s[0] == MINUS {
            new_str.push('-');
        }
        s = &s[1..];
    }

    for i in 0..s.len() {
        if s[i] >= ZERO && s[i] <= NINE {
            new_str.push(s[i] as char);
        } else {
            break;
        }
    }

    // if the string is empty or only contains a sign return 0
    if new_str.len() == 0 || (new_str.len() == 1 && new_str == "-") {
        return 0;
    }

    match new_str.parse::<i32>() {
        Ok(int) => int,
        Err(err) => {
            if err.kind() == &std::num::IntErrorKind::PosOverflow {
                return i32::MAX;
            }
            if err.kind() == &std::num::IntErrorKind::NegOverflow {
                i32::MIN
            } else {
                0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi() {
        assert_eq!(my_atoi("42".to_string()), 42);
        assert_eq!(my_atoi("   -42".to_string()), -42);
        assert_eq!(my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(my_atoi("words and 987".to_string()), 0);
        assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
        assert_eq!(my_atoi("".to_string()), 0);
        assert_eq!(my_atoi(" ".to_string()), 0);
        assert_eq!(my_atoi("  ".to_string()), 0);
        assert_eq!(my_atoi("  -  ".to_string()), 0);
        assert_eq!(my_atoi("  -  1".to_string()), 0);
        assert_eq!(my_atoi("  -  1  ".to_string()), 0);
        assert_eq!(my_atoi("  -  1  2".to_string()), 0);
        assert_eq!(my_atoi("  -  1  2  ".to_string()), 0);
        assert_eq!(my_atoi("  -  1  2  3".to_string()), 0);
        assert_eq!(my_atoi("  -  1  2  3  ".to_string()), 0);
        assert_eq!(my_atoi("  -  1  2  3  4".to_string()), 0);
        assert_eq!(my_atoi("  -  1  2  3  4  ".to_string()), 0);
        assert_eq!(my_atoi("  -  1  2  3  4  5".to_string()), 0);
        assert_eq!(my_atoi("  -  1  2  3  4  5  ".to_string()), 0);
        assert_eq!(my_atoi("  -  1  2  3  4  5  6".to_string()), 0);
        assert_eq!(my_atoi("  -  1  2  3".to_string()), 0);
    }

    #[test]
    fn test_num_bytes() {
        println!("-: {:?}", "-".as_bytes());
        println!("+: {:?}", "+".as_bytes());
        println!("0: {:?}", "0".as_bytes());
        println!("9: {:?}", "9".as_bytes());
        println!("i32 Max: {:?}", i32::MAX.to_string());
        println!("{}", "2147483648".clamp("-2147483647", "2147483647"))
    }
}
