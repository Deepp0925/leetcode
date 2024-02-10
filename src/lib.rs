mod solutions_1_30;

pub fn trap(height: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = height.len() - 1;
    let mut lmax = height[l];
    let mut rmax = height[r];
    let mut res = 0;
    while l < r {
        res += if lmax < rmax {
            l += 1;
            lmax = lmax.max(height[l]);
            lmax - height[l]
        } else {
            r -= 1;
            rmax = rmax.max(height[r]);
            rmax - height[r]
        }
    }

    res
}

#[test]
fn test_fn() {
    assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    // assert_eq!(trap(vec![7, 8, 9, 10, 11]), 1);
}
