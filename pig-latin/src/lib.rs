pub fn translate(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(input: &str) -> String {
    let consonant_count = count_consonants(input);
    let (head, tail) = input.split_at(consonant_count);
    format!("{}{}ay", tail, head)
}

fn count_consonants(input: &str) -> usize {
    if is_vowel(&input.chars().next().expect("input should not be empty!"))
        || input.starts_with("xr")
        || input.starts_with("yt")
    {
        return 0;
    }

    let mut chars = input.chars().peekable();
    let mut head = Vec::new();
    while let Some(current) = chars.next() {
        let next = chars.peek();
        if is_vowel(&current)
            || current == 'q' && next == Some(&'u')
            || current == 'y' && !head.is_empty()
        {
            break;
        }

        head.push(current);
    }

    let head_count = head.len();
    let tail: String = input.chars().skip(head_count).collect();
    if tail.starts_with("qu") {
        head_count + 2
    } else {
        head_count
    }
}

fn is_vowel(letter: &char) -> bool {
    matches!(letter, 'a' | 'e' | 'i' | 'o' | 'u')
}
