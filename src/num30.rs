pub fn s(s: String, words: Vec<String>) -> Vec<usize> {
    fn permute(arr: &mut Vec<String>, start: usize) {
        if start == arr.len() - 1 {
            return;
        }

        for i in start..arr.len() {
            arr.swap(start, i);
            permute(arr, start + 1);
            arr.swap(start, i);
        }
    }
    todo!()
}

fn permute(arr: &mut Vec<String>, start: usize) {
    if start == arr.len() - 1 {
        return;
    }

    for i in start..arr.len() {
        arr.swap(start, i);
        permute(arr, start + 1);
        arr.swap(start, i);
    }
}

#[test]
fn test_permute() {
    let mut strs = vec!["ab".to_string(), "cd".to_string(), "ef".to_string()];

    permute(&mut strs, 0);

    println!("{strs:?}");
}
