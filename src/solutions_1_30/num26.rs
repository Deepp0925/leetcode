pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    nums.dedup();
    nums.len() as i32
}

#[test]
fn test_remove_duplicates() {
    let mut nums1 = vec![1, 1, 2];
    assert_eq!(remove_duplicates(&mut nums1), 2);
    // assert_eq!(nums1, vec![1, 2]);

    let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates(&mut nums2), 5);
    // assert_eq!(nums2, vec![0, 1, 2, 3, 4]);

    let mut nums3 = vec![1, 2];
    assert_eq!(remove_duplicates(&mut nums3), 2);

    let mut nums3 = vec![1, 2, 3];
    assert_eq!(remove_duplicates(&mut nums3), 3);
}
