pub fn raindrops(n: u32) -> String {
    let pairs = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let result = pairs
        .iter()
        .filter(|(i, _)| n % i == 0)
        .map(|(_, s)| s.to_string())
        .collect::<String>();

    if !result.is_empty() {
        result
    } else {
        n.to_string()
    }
}
