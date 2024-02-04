pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&x| x != val);
    nums.len() as i32
}

#[test]
fn test_remove_element() {
    let mut nums1 = vec![3, 2, 2, 3];
    assert_eq!(remove_element(&mut nums1, 3), 2);
    // assert_eq!(nums1, vec![2, 2]);

    let mut nums2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(remove_element(&mut nums2, 2), 5);
    // assert_eq!(nums2, vec![0, 1, 3, 0, 4]);
}
