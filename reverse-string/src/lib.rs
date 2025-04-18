use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // 3rd attempt - using unicode_segmentation to handle grapheme clusters
    input.graphemes(true).rev().collect::<String>()
}

pub fn reverse_simple(input: &str) -> String {
    // 2nd attempt - recall that collect is really powerful!
    // A lot of times we don't have to reach out to `fold`
    input.chars().rev().collect::<String>()
}

fn reverse_old(input: &str) -> String {
    // 1st attempt
    input.chars().rev().fold("".to_string(), |mut acc, elem| {
        acc.push(elem);
        acc
    })
}
