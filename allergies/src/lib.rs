pub struct Allergies(Vec<Allergen>);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    #[rustfmt::skip]
    fn all() -> [Self; 8] {
        use Allergen::*;
        [Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats]
    }

    fn is_in_score(&self, score: u32) -> bool {
        // Does the single bit on the allergen match
        // any of the bits in the score?
        score & (*self as u32) != 0
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        // For each allergen index, we grab its score and check if its
        // (single) bit matches one of the bits on the overall score.
        // If so, we map it into the proper allergen.
        let allergens = Allergen::all();
        Allergies(
            (0..8)
                .filter_map(|idx| {
                    let allergen = allergens[idx];
                    if allergen.is_in_score(score) {
                        Some(allergen)
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.0.clone()
    }
}
