pub fn length_of_longest_substring(s: String) -> i32 {
    let mut max = 0;
    // convert the string to a vector of chars
    let chars: Vec<char> = s.chars().collect();
    let mut set = Vec::with_capacity(chars.len());

    if chars.is_empty() {
        return 0;
    }

    let chars = chars.iter();

    for char in chars {
        let contains = set.contains(char);

        if contains {
            if max < set.len() {
                max = set.len();
            }
            // if the char at first index is the same as the current char
            // then remove the first char
            if char == &set[0] {
                set.remove(0);
            } else {
                // remove the first char until the first char is the same as the current char
                while set[0] != *char {
                    set.remove(0);
                }
                // remove the first char
                set.remove(0);
            }

            set.push(*char);
        } else {
            set.push(*char);
        }
    }

    if set.len() > max {
        max = set.len();
    }

    max as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("".to_string()), 0);
        assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
    }
}
