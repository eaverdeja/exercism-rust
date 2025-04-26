pub fn encode(source: &str) -> String {
    let mut chars = source.chars().peekable();
    let mut count = 0;
    let mut res = String::new();

    while let Some(curr) = chars.next() {
        count += 1;
        if Some(&curr) != chars.peek() {
            if count > 1 {
                res.push_str(&count.to_string());
            }
            res.push(curr);
            count = 0;
        }
    }

    res
}

pub fn decode(source: &str) -> String {
    let chars = source.chars();
    let mut numeric_chars = String::new();
    let mut res = String::new();

    for curr in chars {
        if curr.is_ascii_digit() {
            numeric_chars.push(curr);
            continue;
        }

        if !numeric_chars.is_empty() {
            let num = numeric_chars
                .parse::<usize>()
                .expect("expected valid digit sequence");
            res.push_str(&curr.to_string().repeat(num - 1));
            numeric_chars.clear();
        }
        res.push(curr);
    }

    res
}
