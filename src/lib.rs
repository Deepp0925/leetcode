mod solutions_1_30;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut index = -1;
    let mut left = 0;
    let mut right = nums.len() - 1;

    while right >= left {
        let mid = (right + left) / 2;

        if nums[mid] == target {
            index = mid as i32;
            break;
        }

        // number is between increasing order
        if nums[left] <= nums[mid] {
            if target >= nums[left] && target < nums[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else if target <= nums[right] && target > nums[mid] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    index
}

#[test]
fn test_fn() {
    assert_eq!(search(vec![5, 6, 7, 8, 0, 1, 2, 4], 0), 4);
    assert_eq!(search(vec![5, 6, 7, 0, 1, 2, 3, 4], 7), 2);
    assert_eq!(search(vec![1], 0), -1);
    assert_eq!(search(vec![3, 5, 1], 0), -1);
    assert_eq!(search(vec![5, 1, 3], 2), -1);
    assert_eq!(search(vec![7, 8, 1, 2, 3, 4, 5, 6], 2), 3);
}
