/// Properties: Stable sort, in place sort
/// Runtime:
/// O(log n) for all
/// Description: Brings the biggest item to the end of the array, recursively do this.
/// Steps:
/// 1. divide
/// 2. sort
/// 3. combine
///
/// Recursive merge sort
pub fn merge_sort(arr: Vec<i32>) -> Vec<i32>{
    let mid = arr.len() / 2;
    if mid == 0 {
        return arr;
    }

    let l_arr = merge_sort(arr[..mid].to_vec());
    let r_arr = merge_sort(arr[mid..].to_vec());

    return merge(l_arr, r_arr);
}

fn merge(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::with_capacity(arr1.len() + arr2.len());
    let mut left = 0;
    let mut right = 0;

    // Compare element and insert back to result array
    while left < arr1.len() && right < arr2.len() {
        if arr1[left] <= arr2[right] {
            res.push(arr1[left]);
            left += 1;
        } else {
            res.push(arr2[right]);
            right += 1;
        }
    }

    if left < arr1.len() {
        res.extend_from_slice(&arr1[left..]);
    }
    if right < arr2.len() {
        res.extend_from_slice(&arr2[right..]);
    }

    return res;
}

// Does this in place instead
pub fn iterative_merge_sort(mut arr: &mut Vec<i32>, l: usize, mid: usize, r: usize) {
    let mid = (l + r) / 2;
    if r - l <= 1 {
        return;
    }

    iterative_merge_sort(&mut arr, l, (l + mid)/2, mid);
    iterative_merge_sort(&mut arr, mid, (r + mid)/2, r);

    iterative_merge(&mut arr, l, mid, r);
}

pub fn iterative_merge(arr: &mut Vec<i32>, l: usize, mid: usize, r: usize) {
    let mut left_pos = l;
    let mut right_pos = mid;
    let mut res_arr = arr.clone();

    let mut pos = l;
    while left_pos < mid && right_pos < r {
        if arr[left_pos] <= arr[right_pos] {
            res_arr[pos] = arr[left_pos];
            pos += 1;
            left_pos +=1;
        } else {
            res_arr[pos] = arr[right_pos];
            pos += 1;
            right_pos +=1;
        }
    }

    if left_pos < mid {
        res_arr[pos..r].clone_from_slice(&arr[left_pos..mid]);
    }
    if right_pos < r {
        res_arr[pos..r].clone_from_slice(&arr[right_pos..r]);
    }

    arr.clone_from(&res_arr);
}

#[cfg(test)]
mod test {
    use crate::base_test;
    use super::*;
    fn merge_sort_wrap(arr: &mut [i32]) {
        let merged = merge_sort(arr.to_vec());
        for i in 0..merged.len() {
            arr[i] = merged[i];
        }
    }
    fn iterative_merge_sort_wrap(arr: &mut [i32]) {
        let size = arr.len();
        iterative_merge_sort(&mut arr.to_vec(), 0, size / 2, size);
    }
    base_test!(merge_sort_wrap);

    #[test]
    pub fn test6() {
        let mut test_case_2 = vec![2, 1, 3, 5, 4, 6];
        let result_2 = vec![1, 2, 3, 4, 5, 6];
        iterative_merge_sort(&mut test_case_2, 0, 6/2, 6);
        assert_eq!(test_case_2, result_2);
    }
}
