use std::fmt::Display;

pub trait Luhn<T> {
    fn valid_luhn(&self) -> bool;
}

impl<T: Display> Luhn<T> for T {
    fn valid_luhn(&self) -> bool {
        let filtered: Vec<char> = self
            .to_string()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        if filtered.len() <= 1 || filtered.iter().any(|c| !c.is_ascii_digit()) {
            return false;
        }

        let sum: u32 = filtered
            .iter()
            .filter_map(|c| c.to_digit(10))
            .rev()
            .enumerate()
            .fold(0, |acc, (idx, digit)| {
                if idx % 2 != 0 {
                    let doubled = digit * 2;
                    acc + if doubled > 9 { doubled - 9 } else { doubled }
                } else {
                    acc + digit
                }
            });

        sum % 10 == 0
    }
}
