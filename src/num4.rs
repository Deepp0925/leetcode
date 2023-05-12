use std::collections::BinaryHeap;
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // get the length of two arrays
    let m = nums1.len();
    let n = nums2.len();

    let sum_mn = m + n;

    // is the sum of two arrays is odd or even
    // if odd, the median is the middle number
    // if even, the median is the average of the middle two numbers
    let is_odd = sum_mn % 2 == 1;

    // the median index of the merged array
    let median_index = sum_mn / 2;

    // the merged array
    let mut merged = Vec::with_capacity(median_index + 1);

    // the index of nums1
    let mut i = 0;
    // the index of nums2
    let mut j = 0;

    // merge two arrays
    while merged.capacity() != merged.len() {
        // reached the end of nums1
        if i == m {
            merged.push(nums2[j]);
            j += 1;
            continue;
        }

        // reached the end of nums2
        if j == n {
            merged.push(nums1[i]);
            i += 1;
            continue;
        }

        if nums1[i] < nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
        }
    }

    let last_index = merged.len() - 1;
    // if the sum of two arrays is odd, return the middle number
    if is_odd {
        merged[last_index] as f64
    } else {
        // if the sum of two arrays is even, return the average of the middle two numbers
        (merged[last_index - 1] + merged[last_index]) as f64 / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![0, 0], vec![0, 0]), 0.0);
        assert_eq!(find_median_sorted_arrays(vec![], vec![1]), 1.0);
        assert_eq!(find_median_sorted_arrays(vec![2], vec![]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2, 7]), 2.5);
        assert_eq!(find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5, 6]), 3.5);
        assert_eq!(
            find_median_sorted_arrays(vec![1, 2, 3], vec![4, 5, 6, 7]),
            4.0
        );
        assert_eq!(find_median_sorted_arrays(vec![1, 1, 1], vec![2, 2]), 1.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 1, 1], vec![2, 2, 2]), 1.5);
        assert_eq!(
            find_median_sorted_arrays(vec![1, 6, 17], vec![3, 4, 5, 6]),
            5.0
        );
        assert_eq!(
            find_median_sorted_arrays(vec![1, 3, 6, 8], vec![2, 4, 5, 7]),
            4.5
        );

        assert_eq!(
            find_median_sorted_arrays(vec![1, 3, 6], vec![2, 4, 5, 7]),
            4.0
        );
    }
}
