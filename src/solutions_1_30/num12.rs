pub fn int_to_roman(mut num: i32) -> String {
    let mut ans = String::new();

    while num > 0 {
        if num >= 1000 {
            ans.push('M');
            num -= 1000;
        } else if num >= 900 {
            ans.push_str("CM");
            num -= 900;
        } else if num >= 500 {
            ans.push('D');
            num -= 500;
        } else if num >= 400 {
            ans.push_str("CD");
            num -= 400;
        } else if num >= 100 {
            ans.push('C');
            num -= 100;
        } else if num >= 90 {
            ans.push_str("XC");
            num -= 90;
        } else if num >= 50 {
            ans.push('L');
            num -= 50;
        } else if num >= 40 {
            ans.push_str("XL");
            num -= 40;
        } else if num >= 10 {
            ans.push('X');
            num -= 10;
        } else if num >= 9 {
            ans.push_str("IX");
            num -= 9;
        } else if num >= 5 {
            ans.push('V');
            num -= 5;
        } else if num >= 4 {
            ans.push_str("IV");
            num -= 4;
        } else {
            ans.push('I');
            num -= 1;
        }
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(int_to_roman(3), "III".to_string());
        assert_eq!(int_to_roman(4), "IV".to_string());
        assert_eq!(int_to_roman(9), "IX".to_string());
        assert_eq!(int_to_roman(58), "LVIII".to_string());
        assert_eq!(int_to_roman(1994), "MCMXCIV".to_string());
        assert_eq!(int_to_roman(3999), "MMMCMXCIX".to_string());
    }
}
