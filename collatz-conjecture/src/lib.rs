pub fn collatz(n: u64) -> Option<u64> {
    let increment_step = |x| x + 1;
    match n {
        0 => None,
        1 => Some(0),
        even if even % 2 == 0 => {
            let next = even / 2;
            collatz(next).map(increment_step)
        }
        odd => {
            // checked_* prevents stack overflows
            let next = odd.checked_mul(3)?.checked_add(1)?;
            collatz(next).map(increment_step)
        }
    }
}
