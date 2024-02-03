pub fn str_str(haystack: String, needle: String) -> i32 {
    if let Some(index) = haystack.find(&needle) {
        index as i32
    } else {
        -1
    }
}

#[test]
fn test_find_occurence() {
    assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
    assert_eq!(str_str("mississippi".to_string(), "issi".to_string()), 1);
    println!("{}", 7 - (-3_i32).abs())
}
