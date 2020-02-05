/// Properties: Stable sort, in place sort
/// Runtime:
/// - Best-case: O(n), already sorted
/// - Average-case: O(n^2), randomly sample
/// - Worst-case: O(n^2)
/// Description: Brings the biggest item to the end of the array, recursively do this.
pub fn bubble_sort(arr: &mut [i32]) {

    for i in 0..arr.len() {

        // Progressively shrink the end of the sorting area by 1
        for j in 0..arr.len() - 1 - i {

            // Do in place swaps whenever the current element in bigger than the next.
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
