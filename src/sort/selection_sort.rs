/// Runtime:
/// - O(n^2)
/// Description: Looks through entire list for minimum element through range 0..list_length
/// shrinks list by 1 i.e. range 1..list_length
/// repeat
/// usually the worst sort
pub fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        let mut current_min = i;
        for j in i..arr.len() {
            if arr[j] < arr[current_min] {
                current_min = j;
            }
        }
        arr.swap(i, current_min);
    }
}

#[cfg(test)]
mod test {
    use crate::base_test;
    use super::*;
    base_test!(selection_sort);
}
