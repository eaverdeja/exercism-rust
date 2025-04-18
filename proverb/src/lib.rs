use std::iter;

pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        None => String::new(),
        Some(want) => {
            let punchline = format!("And all for the want of a {want}.");

            let pairs = list.windows(2).map(|w| (w[0], w[1]));
            let verses =
                pairs.map(|(one, other)| format!("For want of a {one} the {other} was lost."));

            verses
                .chain(iter::once(punchline))
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}
