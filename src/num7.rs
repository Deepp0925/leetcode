pub fn reverse(x: i32) -> i32 {
    let mut x = x;
    let mut res = 0;

    while x != 0 {
        let pop = x % 10;
        x /= 10;

        if res > i32::MAX / 10 || (res == i32::MAX / 10 && pop > 7) {
            return 0;
        }

        if res < i32::MIN / 10 || (res == i32::MIN / 10 && pop < -8) {
            return 0;
        }

        res = res * 10 + pop;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(123), 321);
        assert_eq!(reverse(-123), -321);
        assert_eq!(reverse(120), 21);
        assert_eq!(reverse(0), 0);
        assert_eq!(reverse(1534236469), 0);
    }
}
