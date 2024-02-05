pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if divisor == 1 {
        return i32::min(i32::max(dividend, i32::MIN), i32::MAX);
    } else if divisor == -1 && dividend == i32::MIN {
        return i32::MAX;
    }

    let subtracting = dividend.is_negative() ^ divisor.is_negative();

    let mut num = (dividend as i64).abs();
    let den = (divisor as i64).abs();
    let mut result = 0;
    while num >= den {
        let mut temp_den = den;
        let mut multiple = 1;
        while num >= temp_den << 1 {
            temp_den <<= 1;
            multiple <<= 1;
        }

        num -= temp_den;
        result += multiple;
    }

    if subtracting {
        format!("-{result}").parse().unwrap()
    } else {
        result
    }
}

#[test]
fn test_fn() {
    assert_eq!(divide(10, 3), 3);
    assert_eq!(divide(7, -3), -2);
}
