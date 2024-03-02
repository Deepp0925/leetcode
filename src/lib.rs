mod solutions_1_30;

pub fn add_binary(a: String, b: String) -> String {
    if a == "0" {
        return b;
    } else if b == "0" {
        return a;
    }

    // ensures the a always greater than or equal to b
    let (mut a, mut b) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    let mut new_string = String::new();

    let mut carry = false;
    while let Some(last_a) = a.pop() {
        if let Some(last_b) = b.pop() {
            match (last_a, last_b) {
                ('0', '0') => {
                    if carry {
                        carry = false;
                        new_string.insert(0, '1');
                    } else {
                        new_string.insert(0, '0');
                    }
                }
                ('1', '1') => {
                    if carry {
                        new_string.insert(0, '1');
                    } else {
                        carry = true;
                        new_string.insert(0, '0');
                    }
                }
                ('1', '0') | ('0', '1') => {
                    if carry {
                        new_string.insert(0, '0');
                    } else {
                        new_string.insert(0, '1');
                    }
                }
                _ => unreachable!(),
            }
        } else {
            match (carry, last_a) {
                (true, '1') => {
                    new_string.insert(0, '0');
                }
                (false, '1') | (true, '0') => {
                    new_string.insert(0, '1');
                    carry = false;
                }
                (false, '0') => {
                    new_string.insert(0, '0');
                }
                _ => unreachable!("heere"),
            }
        }
    }

    if &new_string[0..1] == "0" || carry {
        new_string.insert(0, '1');
    }

    new_string
}

#[test]
fn test_fn() {
    assert_eq!(
        add_binary("100".to_string(), "110010".to_string()),
        "110110".to_string()
    );
}
