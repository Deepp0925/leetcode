/// Two sum complementry problem
mod num1;
/// Revese Node Linked List
mod num2;
/// Longest Substring Without Repeating Characters
mod num3;
/// Median of Two Sorted Arrays
mod num4;
/// Longest Palindromic Substring
mod num5;
/// ZigZag Conversion
mod num6;
/// Reverse Integer
mod num7;
/// String to Integer (atoi)
mod num8;
/// Palindrome Number
mod num9;

/// Regular Expression Matching
mod num10;
/// Container With Most Water
mod num11;
/// Integer to Roman
mod num12;
/// Roman to Integer
mod num13;
/// Longest Common Prefix
mod num14;
/// 3Sum
mod num15;
/// 3Sum Closest
mod num16;
/// Letter Combinations of a Phone Number
mod num17;
/// 4Sum
mod num18;

pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut bool_arr = Vec::with_capacity(len);
    bool_arr.push(true);

    for i in 1..len {
        let curr = arr[i];
        let prev = arr[i - 1];

        if curr < prev {
            bool_arr.push(false);
        } else {
            bool_arr.push(true);
        }
    }

    // check if there is any false in the bool_arr
    if bool_arr.iter().all(|&x| x == true) {
        return 0;
    }

    // get the first and last index of the false
    let mut first = 0;
    let mut last = 0;

    for i in 0..len {
        if bool_arr[i] == false {
            first = i;
            break;
        }
    }

    for i in (first..len).rev() {
        if bool_arr[i] == false {
            last = i;
            break;
        }
    }

    let mut two_arrs: Vec<Vec<i32>> = vec![];
    two_arrs.push(vec![]);
    for i in 0..first {
        two_arrs[0].push(arr[i]);
    }

    two_arrs.push(vec![]);
    for i in last..len {
        two_arrs[1].push(arr[i]);
    }

    println!("{:?}", two_arrs);

    if two_arrs[0].is_empty() && two_arrs[1].is_empty() {
        return len as i32 - 1;
    } else if two_arrs[0].is_empty() {
        return len as i32 - two_arrs[1].len() as i32;
    } else if two_arrs[1].is_empty() {
        return len as i32 - two_arrs[0].len() as i32;
    }

    // remove the last element of the first array
    // as long as the last element of the first array is greater than the first element of the second array
    let mut i = two_arrs[0].len() - 1;
    let mut j = 0;

    while i > 0 && j < two_arrs[1].len() {
        if two_arrs[0][i] > two_arrs[1][j] {
            two_arrs[0].remove(i);
            i -= 1;
        } else {
            break;
        }
    }

    let flattened = two_arrs.into_iter().flatten().collect::<Vec<i32>>();

    println!("{:?}", flattened);

    len as i32 - flattened.len() as i32
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_find_length_of_shortest_subarray() {
        let arr: Vec<i32> = vec![1, 2, 3, 10, 4, 2, 3, 5];
        let ans = 3;

        assert_eq!(super::find_length_of_shortest_subarray(arr), ans);

        let arr: Vec<i32> = vec![5, 4, 3, 2, 1];
        let ans = 4;
        assert_eq!(super::find_length_of_shortest_subarray(arr), ans);

        let arr: Vec<i32> = vec![1, 2, 3];
        let ans = 0;
        assert_eq!(super::find_length_of_shortest_subarray(arr), ans);

        let arr: Vec<i32> = vec![2, 2, 2, 1, 1, 1];
        let ans = 3;
        assert_eq!(super::find_length_of_shortest_subarray(arr), ans);

        let arr = vec![13, 0, 14, 7, 18, 18, 18, 16, 8, 15, 20];
        let ans = 8;
        assert_eq!(super::find_length_of_shortest_subarray(arr), ans);

        let arr: Vec<i32> = vec![1, 3, 2, 4];
        let ans = 1;
        assert_eq!(super::find_length_of_shortest_subarray(arr), ans);

        let arr = vec![1, 2, 3, 10, 0, 7, 8, 9];
        let ans = 2;
        assert_eq!(super::find_length_of_shortest_subarray(arr), ans);

        let arr = vec![1, 2, 3, 3, 10, 1, 3, 3, 5];
        let ans = 2;
        assert_eq!(super::find_length_of_shortest_subarray(arr), ans);
    }

    // #[test]
    // fn num_ways() {
    //     let s = "1001|000101|00110".to_string();
    //     let as_bytes = s.as_bytes();
    //     let num_ways = 4;
    //     let len = s.len();
    //     // store the indices of 1s
    //     let mut ones = Vec::with_capacity(len);

    //     for i in 0..len {
    //         if as_bytes[i] as char == '1' {
    //             ones.push(i);
    //         }
    //     }

    //     let mut ans = 0;
    //     let len = ones.len();

    //     // if the len is not divisible by 3, then there is no way to split the string
    //     if len % 3 != 0 {
    //         ans = 0;
    //     } else {
    //         for i in 0..len {
    //             let j = i + 1;
    //             let k = j + 1;

    //             if j < len && k < len {
    //                 let first = ones[j] - ones[i];
    //                 let second = ones[k] - ones[j];
    //                 let third = len - ones[k];

    //                 if first == second && second == third {
    //                     ans += 1;
    //                 }
    //             }
    //         }
    //     }

    //     todo!()
    // }
}
