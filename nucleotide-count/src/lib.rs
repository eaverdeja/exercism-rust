use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut counts = nucleotide_counts(dna)?;
    counts.remove(&nucleotide).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let counts: HashMap<char, usize> = ['A', 'C', 'G', 'T'].iter().map(|&n| (n, 0)).collect();

    dna.chars().try_fold(counts, |mut acc, nucleotide| {
        match acc.get_mut(&nucleotide) {
            Some(count) => {
                *count += 1;
                Ok(acc)
            }
            None => Err(nucleotide),
        }
    })
}
