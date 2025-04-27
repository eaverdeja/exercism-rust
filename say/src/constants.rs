#[rustfmt::skip]
pub mod names {
    pub const SMALL: [&str; 20] = ["zero", "one", "two", "three", "four", "five", "six", 
    "seven", "eight", "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
    "sixteen", "seventeen", "eighteen", "nineteen"];

    pub const TENS: [&str; 10] = ["zero", "ten", "twenty", "thirty", "forty", "fifty", 
                            "sixty", "seventy", "eighty", "ninety"];

    pub const SCALES: [&str; 7] = ["hundred", "thousand", "million", "billion", 
    "trillion", "quadrillion", "quintillion"]; // enough for u64::MAX
}
