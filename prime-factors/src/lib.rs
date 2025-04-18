pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut divisor: u64 = 2;
    let mut rest = n;

    while rest != 1 {
        rest = match rest % divisor {
            0 => {
                factors.push(divisor);
                rest / divisor
            }
            _ => {
                divisor += 1;
                rest
            }
        }
    }

    factors
}
