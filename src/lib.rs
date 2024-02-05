mod solutions_1_30;

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![-1, -1];
    }

    let index = nums.binary_search(&target);

    if index.is_err() {
        return vec![-1, -1];
    }

    let index = index.unwrap() as i32;

    let mut leftmost = index;
    let mut rightmost = index;
    let mut left = index - 1;
    let mut right = index + 1;

    loop {
        if left >= 0 && nums[left as usize] == target {
            leftmost = left;
        }

        if right < nums.len() as i32 && nums[right as usize] == target {
            rightmost = right;
        }

        if rightmost != right && leftmost != left {
            break;
        }

        left -= 1;
        right += 1;
    }

    vec![leftmost, rightmost]
}

#[test]
fn test_fn() {
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), [3, 4]);
    assert_eq!(search_range(vec![2, 2], 2), [0, 1]);
}
