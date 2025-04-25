use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Palindrome {
    value: u64,
    min: u64,
    max: u64,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        (self.min..=self.value.isqrt())
            .filter_map(|i| {
                if self.value % i != 0 {
                    return None;
                }
                let factor = self.value / i;

                (self.min..=self.max)
                    .contains(&factor)
                    .then_some((i, factor))
            })
            .collect()
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut min_product = u64::MAX;
    let mut max_product = 0;
    let mut found = false;

    for i in min..=max {
        if i % 10 == 0 {
            continue;
        }
        // Start from i to avoid double-work
        for j in i..=max {
            if j % 10 == 0 {
                continue;
            }
            let product = i * j;
            if is_palindrome(product) {
                found = true;
                min_product = min_product.min(product);
                max_product = max_product.max(product);
            }
        }
    }

    if !found {
        return None;
    }

    Some((
        Palindrome {
            value: min_product,
            min,
            max,
        },
        Palindrome {
            value: max_product,
            min,
            max,
        },
    ))
}

fn is_palindrome(value: u64) -> bool {
    let mut reversed = 0;
    let mut original = value;

    while original > 0 {
        reversed = reversed * 10 + original % 10;
        original /= 10;
    }

    value == reversed
}
