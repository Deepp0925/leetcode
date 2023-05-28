pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut left = 0;
    let mut right = height.len() - 1;

    while left < right {
        let area = (right - left) as i32 * std::cmp::min(height[left], height[right]);
        max = std::cmp::max(max, area);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(max_area(vec![1, 1]), 1);
        assert_eq!(max_area(vec![4, 3, 2, 1, 4]), 16);
        assert_eq!(max_area(vec![1, 2, 1]), 2);
        assert_eq!(max_area(vec![1, 2, 4, 3]), 4);
    }
}
