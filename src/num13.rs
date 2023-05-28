pub fn roman_to_int(s: String) -> i32 {
    let mut chars = s.chars();

    let mut ans = 0;

    todo!()
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
