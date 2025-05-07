use std::fmt::{Display, Formatter, Result};

static THOUSANDS: &[&str] = &["", "M", "M", "MMM"];
static HUNDREDS: &[&str] = &[
    "", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM", "M",
];
static TENS: &[&str] = &[
    "", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC", "C",
];
static UNITS: &[&str] = &[
    "", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X",
];

pub struct Roman(String);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(format!(
            "{}{}{}{}",
            THOUSANDS[(num / 1000) as usize],
            HUNDREDS[(num % 1000 / 100) as usize],
            TENS[(num % 100 / 10) as usize],
            UNITS[(num % 10) as usize]
        ))
    }
}
