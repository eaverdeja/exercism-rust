pub fn encrypt(input: &str) -> String {
    let normalized = input
        .to_ascii_lowercase()
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<Vec<char>>();

    if normalized.is_empty() {
        return String::new();
    }

    let (columns, rows) = (1..)
        .map(|i| (normalized.len().div_ceil(i), i))
        .find(|(column, row)| column - row <= 1)
        .expect("could not find (columns, rows) pair");

    (0..columns)
        .map(|col_idx| {
            (0..rows)
                .map(|row_idx| normalized.get(row_idx * columns + col_idx).unwrap_or(&' '))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
