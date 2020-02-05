/// Runtime:
/// - worst case: O(n^2)
/// - average: O(nlogn)
/// Description: Looks through entire list for minimum element through range 0..list_length
/// shrinks list by 1 i.e. range 1..list_length
/// repeat
/// Best case when we pick a good pivot and split roughly by half
pub fn quick_sort(arr: &mut [i32]) {
    let pivot = 0;
    let mut pivot_swap_position = 1;
    if arr.len() == 0 || arr.len() == 1 {
        return
    }

    for i in 1..arr.len() {
        if arr[pivot] > arr[i] {
            arr.swap(i, pivot_swap_position);
            pivot_swap_position += 1;
        }
    }

    arr.swap(pivot, pivot_swap_position - 1);

    if pivot_swap_position < arr.len() - 1 {
        quick_sort(&mut arr[pivot_swap_position..]);
    }
}

#[cfg(test)]
mod test {
    use crate::base_test;
    use super::*;
    base_test!(quick_sort);
}
