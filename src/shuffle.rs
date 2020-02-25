extern crate rand;

use rand::{Rng, thread_rng};

pub fn knuth_shuffle(arr: &mut [i32]) {
    let mut rng = thread_rng();
    let size = arr.len();
    for i in 1..size {
        let newPos = rng.gen_range(0, i);
        arr.swap(i, newPos);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    // Need to improve this since knuth shuffle return an ordered array;
    pub fn test_knuth_shuffle() {
        let mut test_case = [1, 2, 3, 4, 5, 6];
        let not_expected = [1, 2, 3, 4, 5, 6];
        knuth_shuffle(&mut test_case);
        assert_ne!(test_case, not_expected);
    }
}
