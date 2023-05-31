pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut map = std::collections::HashMap::with_capacity(9);
    map.insert('2', vec!["a", "b", "c"]);
    map.insert('3', vec!["d", "e", "f"]);
    map.insert('4', vec!["g", "h", "i"]);
    map.insert('5', vec!["j", "k", "l"]);
    map.insert('6', vec!["m", "n", "o"]);
    map.insert('7', vec!["p", "q", "r", "s"]);
    map.insert('8', vec!["t", "u", "v"]);
    map.insert('9', vec!["w", "x", "y", "z"]);

    let mut combinations: Vec<String> = Vec::new();
    let digits = digits.chars();

    for digit in digits {
        let letters = map.get(&digit).unwrap();
        let mut new_combinations = Vec::new();

        if combinations.is_empty() {
            for letter in letters {
                new_combinations.push(letter.to_string());
            }
        } else {
            for combination in combinations {
                for letter in letters {
                    let mut new_combination = combination.clone();
                    new_combination.push_str(letter);
                    new_combinations.push(new_combination);
                }
            }
        }

        combinations = new_combinations;
    }

    combinations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        assert_eq!(
            letter_combinations("23".to_string()),
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(letter_combinations("".to_string()), Vec::<String>::new());
        assert_eq!(letter_combinations("2".to_string()), vec!["a", "b", "c"]);
        assert_eq!(
            letter_combinations("234".to_string()),
            vec![
                "adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg", "bdh", "bdi",
                "beg", "beh", "bei", "bfg", "bfh", "bfi", "cdg", "cdh", "cdi", "ceg", "ceh", "cei",
                "cfg", "cfh", "cfi"
            ]
        );
    }
}
