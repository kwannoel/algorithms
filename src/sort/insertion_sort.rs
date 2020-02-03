/// Runtime:
/// - O(n^2)
/// Description: Looks through entire list for minimum element through range 0..list_length
/// shrinks list by 1 i.e. range 1..list_length
/// repeat
pub fn insertion_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in 0..i {
            if arr[i - j] < arr[i - j - 1] {
                arr.swap(i - j, i - j - 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::base_test;
    use super::*;
    base_test!(insertion_sort);
}
