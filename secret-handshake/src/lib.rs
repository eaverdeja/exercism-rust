static ACTIONS: &[(u8, &str)] = &[
    (0b0001, "wink"),
    (0b0010, "double blink"),
    (0b0100, "close your eyes"),
    (0b1000, "jump"),
];

pub fn actions(n: u8) -> Vec<&'static str> {
    let mut result: Vec<&'static str> = ACTIONS
        .iter()
        .filter_map(|(mask, action)| match n & *mask {
            m if m == *mask => Some(*action),
            _ => None,
        })
        .collect();

    if n & 0b10000 != 0 {
        result.reverse();
    }

    result
}
