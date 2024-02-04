pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    // shift each window by the word len in every iteration
    let word_len = words[0].len();
    // this is also the window len
    let permutation_len = word_len * words.len();

    let mut result = vec![];

    if permutation_len > s.len() {
        return result;
    }

    let mut mapped_words = std::collections::HashMap::with_capacity(words.len());

    for word in words {
        if let Some(val) = mapped_words.get_mut(&word) {
            *val += 1;
        } else {
            mapped_words.insert(word, 1);
        }
    }

    let has_permutation = |s: &str| -> bool {
        let mut mapped_window_items = std::collections::HashMap::with_capacity(s.len() / word_len);

        for i in (0..s.len()).step_by(word_len) {
            let mini_window = &s[i..i + word_len];
            if let Some(val) = mapped_window_items.get_mut(&mini_window) {
                *val += 1;
            } else {
                mapped_window_items.insert(mini_window, 1);
            }
        }

        for (item, count) in mapped_window_items {
            if let Some(mapped_word_val) = mapped_words.get(item) {
                if &count != mapped_word_val {
                    return false;
                }
            } else {
                return false;
            };
        }

        true
    };

    for i in 0..s.len() {
        let range = i..i + permutation_len;
        if range.end > s.len() {
            break;
        }
        let s = &s[range];
        if has_permutation(s) {
            result.push(i as i32);
        }
    }

    result
}

#[test]
fn test_find_substring() {
    assert_eq!(
        find_substring(
            "barfoothefoobarman".to_string(),
            vec!["foo".to_string(), "bar".to_string()]
        ),
        vec![0, 9]
    );

    assert_eq!(
        find_substring(
            "wordgoodgoodgoodbestword".to_string(),
            vec![
                "word".to_string(),
                "good".to_string(),
                "best".to_string(),
                "word".to_string()
            ]
        ),
        vec![]
    );

    assert_eq!(
        find_substring(
            "barfoofoobarthefoobarman".to_string(),
            vec!["foo".to_string(), "bar".to_string(), "the".to_string(),]
        ),
        vec![6, 9, 12]
    );

    assert_eq!(
        find_substring(
            "wordgoodgoodgoodbestword".to_string(),
            vec![
                "word".to_string(),
                "good".to_string(),
                "best".to_string(),
                "good".to_string()
            ]
        ),
        vec![8]
    );

    assert_eq!(
        find_substring(
            "lingmindraboofooowingdingbarrwingmonkeypoundcake".to_string(),
            vec![
                "fooo".to_string(),
                "barr".to_string(),
                "wing".to_string(),
                "ding".to_string(),
                "wing".to_string()
            ]
        ),
        vec![13]
    );

    assert_eq!(
        find_substring(
            "ababaab".to_string(),
            vec!["ab".to_string(), "ba".to_string(), "ba".to_string(),]
        ),
        vec![1]
    );
}
