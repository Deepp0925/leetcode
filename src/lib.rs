mod solutions_1_30;

pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.retain(|n| n > &0);
    let n = nums.len();
    for i in 0..n {
        while nums[i] <= n as i32 && nums[(nums[i] - 1) as usize] != nums[i] {
            let j = (nums[i] - 1) as usize;
            nums.swap(i, j);
        }
    }
    for (i, elem) in nums.into_iter().enumerate().take(n) {
        let next = (i + 1) as i32;
        if elem != next {
            return next;
        }
    }
    (n + 1) as i32
}

#[test]
fn test_fn() {
    assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
    assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
    assert_eq!(first_missing_positive(vec![7, 8, 9, 10, 11]), 1);
}
