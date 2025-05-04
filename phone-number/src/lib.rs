pub fn number(user_number: &str) -> Option<String> {
    let digits: String = user_number.chars().filter(char::is_ascii_digit).collect();
    let digits_without_country = if digits.starts_with("1") {
        digits.strip_prefix("1")?
    } else {
        &digits
    };

    if digits_without_country.len() != 10 {
        return None;
    }
    if !('2'..='9').contains(&digits_without_country.chars().next()?) {
        return None;
    }
    if !('2'..='9').contains(&digits_without_country.chars().nth(3)?) {
        return None;
    }

    Some(digits_without_country.to_string())
}
