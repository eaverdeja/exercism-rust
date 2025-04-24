// This solution is "nice", but it does start from
// a least likely condition, so it is less efficient
// than a one-liner that short-circuits if the year
// is not divisible by 4.
//
// This does 2 calculations and 2 checks, always.
// The one liner would do 1 calculation and 1 check in most cases.
pub fn is_leap_year(year: u64) -> bool {
    match year % 100 {
        0 => year % 400 == 0,
        _ => year % 4 == 0,
    }
}
