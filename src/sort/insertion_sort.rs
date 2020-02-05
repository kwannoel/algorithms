/// Runtime:
/// - O(n^2)
/// Description: Looks through entire list for minimum element through range 0..list_length
/// shrinks list by 1 i.e. range 1..list_length
/// repeat
pub fn insertion_sort(arr: &mut [i32]) {

    // For each element in the array
    for i in 0..arr.len() {

        // For each element up to i
        for j in 0..i {

            // Starting from last element, i.e. i-0 = i and going to i-(i-1) = 1
            // If the current element is smaller than the previous one, swap them.
            if arr[i - j] < arr[i - j - 1] {
                arr.swap(i - j, i - j - 1);

            // If it isnt, the array is sorted till the current element so exit to loop.
            } else {
                break;
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
