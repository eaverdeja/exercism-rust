#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        validate_sequence(dna, &['G', 'C', 'T', 'A']).map(Dna)
    }

    pub fn into_rna(self) -> Rna {
        let rna = self
            .0
            .chars()
            .map(|ch| match ch {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect::<String>();

        Rna(rna)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        validate_sequence(rna, &['G', 'C', 'U', 'A']).map(Rna)
    }
}

fn validate_sequence(input: &str, valid_nucleotides: &[char]) -> Result<String, usize> {
    input.chars().enumerate().try_fold(
        String::with_capacity(input.len()),
        |mut valid_seq, (idx, nucleotide)| {
            if valid_nucleotides.contains(&nucleotide) {
                valid_seq.push(nucleotide);
                Ok(valid_seq)
            } else {
                Err(idx)
            }
        },
    )
}
