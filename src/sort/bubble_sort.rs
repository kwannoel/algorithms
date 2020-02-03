/// Properties: Stable sort, in place sort
/// Runtime:
/// - Best-case: O(n), already sorted
/// - Average-case: O(n^2), randomly sample
/// - Worst-case: O(n^2)
/// Description: Brings the biggest item to the end of the array, recursively do this.
pub fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::base_test;
    use super::*;
    base_test!(bubble_sort);
}
