pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "");
    if isbn.len() != 10 {
        return false;
    }

    match isbn.chars().enumerate().try_fold(0, |acc, (i, ch)| {
        if ch == 'X' && i == 9 {
            Some(acc + 10)
        } else {
            ch.to_digit(10).map(|digit| acc + digit * (10 - i as u32))
        }
    }) {
        Some(sum) => sum % 11 == 0,
        _ => false,
    }
}
