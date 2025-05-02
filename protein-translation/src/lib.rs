use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    rna.as_bytes()
        .chunks(3)
        .fold_while(
            Some(Vec::with_capacity(rna.len() / 3)),
            |mut proteins, codon| {
                let protein = match codon {
                    b"AUG" => "Methionine",
                    b"UUC" | b"UUU" => "Phenylalanine",
                    b"UUA" | b"UUG" => "Leucine",
                    b"UCU" | b"UCC" | b"UCA" | b"UCG" => "Serine",
                    b"UAU" | b"UAC" => "Tyrosine",
                    b"UGU" | b"UGC" => "Cysteine",
                    b"UGG" => "Tryptophan",
                    b"UAA" | b"UAG" | b"UGA" => return Done(proteins),
                    _ => return Done(None),
                };

                proteins.as_mut().unwrap().push(protein);
                Continue(proteins)
            },
        )
        .into_inner()
}
