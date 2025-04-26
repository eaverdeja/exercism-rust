use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match aliquot_sum(num)?.cmp(&num) {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Less => Some(Classification::Deficient),
        Ordering::Greater => Some(Classification::Abundant),
    }
}

fn aliquot_sum(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        _ => {
            let mut sum = 1; // 1 is always a divisor
            for i in 2..=n.isqrt() {
                if n % i == 0 {
                    // Add divisors
                    sum += i;
                    // Careful not to count the sqrt twice
                    if i != n / i {
                        sum += n / i;
                    }
                }
            }

            Some(sum)
        }
    }
}
