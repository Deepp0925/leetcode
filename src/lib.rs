mod solutions_1_30;

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut res = 0;
    let _ = nums
        .binary_search(&target)
        .map(|pos| res = pos as i32)
        .map_err(|exp_pos| res = exp_pos as i32);

    res
}

#[test]
fn test_fn() {
    assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
}
