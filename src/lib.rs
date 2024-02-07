mod solutions_1_30;

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(pos) => pos as i32,
        Err(pos) => pos as i32,
    }
}

#[test]
fn test_fn() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
}
