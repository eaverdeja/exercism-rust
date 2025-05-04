pub fn get_diamond(target: char) -> Vec<String> {
    ('A'..=target)
        .chain(('A'..target).rev())
        .map(|ch| {
            let border = (target as u8 - ch as u8) as usize;
            let inner = (ch as u8 - b'A') as usize;

            if ch == 'A' {
                format!("{ch: ^b$}", b = border * 2 + 1)
            } else {
                format!(
                    "{ch: >b$}{:i$}{ch: <b$}",
                    ' ',
                    b = border + 1,
                    i = inner * 2 - 1
                )
            }
        })
        .collect()
}
