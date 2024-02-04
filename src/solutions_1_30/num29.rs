pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == i32::MIN && divisor == -1 {
        return i32::MAX;
    }

    let is_negative = dividend.is_negative() ^ divisor.is_negative();

    let mut num = dividend as i64;
    let den = divisor as i64;

    let mut result = 0;

    while num >= den {
        let mut den_copy = den;
        let mut multiple = 1;

        while num >= den_copy << 1 {
            den_copy <<= 1; // << is multiply by 2
            multiple <<= 1;
        }

        num -= den_copy; //ex 15 - 12
        result += multiple; //ex 4, then 1
    }

    if is_negative {
        format!("-{result}").parse().unwrap()
    } else {
        result
    }
}

#[test]
fn test_divide() {
    // println!("{:?}", "-23".parse::<i32>().unwrap());
    // println!("{}", -2147483648_i32.saturating_div(-1))
    println!("{}", 3 << 1)
    // assert_eq!(divide(10, 3), 3);
    // assert_eq!(divide(7, -3), -2);
    // assert_eq!(divide(1, 1), 1);
    // assert_eq!(divide(-2147483648, -1), 2147483647);
    // assert_eq!(divide(-2147483648, 1), -2147483648);
    // assert_eq!(divide(2147483647, 2), 1073741823);
    // assert_eq!(divide(-2147483648, 2), -1073741824)
}
