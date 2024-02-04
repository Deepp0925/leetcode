pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();

    let mut result = Vec::new();

    for (i, a) in nums.iter().enumerate() {
        if i > 0 && a == &nums[i - 1] {
            continue;
        }

        for (j, b) in nums.iter().enumerate().skip(i + 1) {
            if j > i + 1 && b == &nums[j - 1] {
                continue;
            }

            let mut k = j + 1;
            let mut l = nums.len() - 1;

            while k < l {
                let sum = a + b + nums[k] + nums[l];

                if sum == target {
                    result.push(vec![*a, *b, nums[k], nums[l]]);

                    k += 1;
                    l -= 1;

                    while k < l && nums[k] == nums[k - 1] {
                        k += 1;
                    }

                    while k < l && nums[l] == nums[l + 1] {
                        l -= 1;
                    }
                } else if sum < target {
                    k += 1;
                } else {
                    l -= 1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum() {
        assert_eq!(
            four_sum(
                vec![1000000000, 1000000000, 1000000000, 1000000000],
                -294967296
            ),
            Vec::<Vec<i32>>::new()
        );
        assert_eq!(
            four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-1, 0, 0, 1], vec![-2, -1, 1, 2], vec![-2, 0, 0, 2]]
        );
        assert_eq!(four_sum(vec![], 0), Vec::<Vec<i32>>::new());
        assert_eq!(four_sum(vec![0], 0), Vec::<Vec<i32>>::new());
        assert_eq!(four_sum(vec![0, 0, 0, 0], 0), vec![vec![0, 0, 0, 0]]);
        assert_eq!(
            four_sum(vec![1, -2, -5, -4, -3, 3, 3, 5], -11),
            vec![vec![-5, -4, -3, 1]]
        );
    }
}
