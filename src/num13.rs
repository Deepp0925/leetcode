pub fn roman_to_int(s: String) -> i32 {
    let mut chars = s.chars().peekable();

    let mut ans = 0;

    while let Some(char) = chars.next() {
        match char {
            'I' => {
                if let Some('V') = chars.peek() {
                    ans += 4;
                    chars.next();
                } else if let Some('X') = chars.peek() {
                    ans += 9;
                    chars.next();
                } else {
                    ans += 1;
                }
            }
            'V' => {
                ans += 5;
            }
            'X' => {
                if let Some('L') = chars.peek() {
                    ans += 40;
                    chars.next();
                } else if let Some('C') = chars.peek() {
                    ans += 90;
                    chars.next();
                } else {
                    ans += 10;
                }
            }
            'L' => {
                ans += 50;
            }
            'C' => {
                if let Some('D') = chars.peek() {
                    ans += 400;
                    chars.next();
                } else if let Some('M') = chars.peek() {
                    ans += 900;
                    chars.next();
                } else {
                    ans += 100;
                }
            }
            'D' => {
                ans += 500;
            }
            'M' => {
                ans += 1000;
            }
            _ => unreachable!(),
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("IX".to_string()), 9);
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
