mod sort;
mod test_utils;
use crate::sort::merge_sort::*;
fn main() {
    println!("Hello, world!");
    iterative_merge(&mut vec![2, 3], 0, 1, 2);
}
