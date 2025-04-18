/*
This solution is O(n) since we have to iterate over the range of numbers
There are O(1) solutions that take advantage of mathematical formulas
See: https://exercism.org/tracks/rust/exercises/difference-of-squares/solutions/bobahop
*/

pub fn square_of_sum(n: u32) -> u32 {
    (0..=n).sum::<u32>().pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (0..=n).map(|i| i.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
