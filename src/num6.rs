pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    let len = s.len();
    let mut arr = vec![Vec::with_capacity(len); num_rows as usize];

    let mut current_row = 0;
    let mut down = true;

    for c in s.chars() {
        arr[current_row].push(c);

        if !down {
            current_row -= 1;
            if current_row == 0 {
                down = true;
            }
        } else {
            current_row += 1;
            if current_row == num_rows as usize - 1 {
                down = false;
            }
        }
    }

    let mut result = String::with_capacity(len);
    // flatten the arr
    for i in 0..num_rows as usize {
        result.push_str(arr[i].iter().collect::<String>().as_str());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR");
        assert_eq!(convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI");
        assert_eq!(convert("PAYPALISHIRING".to_string(), 1), "PAYPALISHIRING");
        assert_eq!(convert("PAYPALISHIRING".to_string(), 2), "PYAIHRNAPLSIIG");
        assert_eq!(convert("PAYPALISHIRING".to_string(), 5), "PHASIYIRPLIGAN");
        assert_eq!(convert("A".to_string(), 1), "A");
    }
}
