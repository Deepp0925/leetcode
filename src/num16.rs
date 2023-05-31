pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut closest = nums[0] + nums[1] + nums[2];
    let mut diff = (closest - target).abs();

    for i in 0..nums.len() - 2 {
        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            let curr_diff = (sum - target).abs();

            if curr_diff < diff {
                diff = curr_diff;
                closest = sum;
            }

            if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    closest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_closest() {
        assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
        assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
        assert_eq!(three_sum_closest(vec![1, 1, 1, 0], -100), 2);
    }
}
