pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let num_of_digits = digits.len() as u32;
    let sum_of_raised_digits: u32 = digits.iter().map(|digit| digit.pow(num_of_digits)).sum();

    num == sum_of_raised_digits
}
