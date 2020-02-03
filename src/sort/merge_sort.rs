/// Properties: Stable sort, in place sort
/// Runtime:
/// O(log n) for all
/// Description: Brings the biggest item to the end of the array, recursively do this.
/// Steps:
/// 1. divide
/// 2. sort
/// 3. combine
pub fn merge_sort(arr: &mut [i32]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    // create an array to store intermediate result
    let mut res = arr.to_vec();

    // Merge the two piles.
    merge(&arr[..mid], &arr[mid..], &mut res[..]);

    arr.copy_from_slice(&res);
}

fn merge(arr1: &[i32], arr2: &[i32], res: &mut [i32]) {
    let mut left = 0;
    let mut right = 0;
    let mut index = 0;

    // Compare element and insert back to result array
    while left < arr1.len() && right < arr2.len() {
        if arr1[left] <= arr2[right] {
            res[index] = arr1[left];
            left += 1;
            index += 1;
        } else {
            res[index] = arr2[right];
            right += 1;
            index += 1;
        }
    }

    if left < arr1.len() {
        res[index..].copy_from_slice(&arr1[left..]);
    }
    if right < arr2.len() {
        res[index..].copy_from_slice(&arr2[right..]);
    }
}

#[cfg(test)]
mod test {
    use crate::base_test;
    use super::*;
    base_test!(merge_sort);
}
