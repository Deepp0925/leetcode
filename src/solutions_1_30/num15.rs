pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return Vec::new();
    }

    let mut map = std::collections::HashMap::with_capacity(nums.len());
    for (i, num) in nums.iter().enumerate() {
        map.insert(num, i);
    }

    let mut three_sum = Vec::new();
    for (i, num) in nums.iter().enumerate() {
        let target = -num;

        for j in i + 1..nums.len() {
            let complement = target - nums[j];
            if let Some(&k) = map.get(&complement) {
                if k != i && k != j {
                    let mut compliments = vec![nums[i], nums[j], nums[k]];
                    compliments.sort_unstable();
                    if !three_sum.contains(&compliments) {
                        three_sum.push(compliments);
                    }
                }
            }
        }
    }

    three_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
        assert_eq!(three_sum(vec![]), Vec::<Vec<i32>>::new());
        assert_eq!(three_sum(vec![0]), Vec::<Vec<i32>>::new());
    }
}
