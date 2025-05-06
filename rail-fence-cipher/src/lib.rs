pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> Self {
        Self {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rail = 0;
        let mut direction = 1;

        text.chars()
            .fold(vec![Vec::new(); self.rails], |mut fence, ch| {
                fence[rail].push(ch);
                rail = (rail as isize + direction) as usize;

                if rail == 0 {
                    direction = 1;
                } else if rail == self.rails - 1 {
                    direction = -1;
                }

                fence
            })
            .into_iter()
            .flatten()
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut rail = 0;
        let mut direction = 1;
        let cipher_len = cipher.chars().count();

        // Create a fence template with the correct
        // positions for each letter
        let fence = (0..cipher_len).fold(
            vec![vec![None; cipher_len]; self.rails],
            |mut fence, pos| {
                fence[rail][pos] = Some(pos);

                if rail == 0 {
                    direction = 1;
                } else if rail == self.rails - 1 {
                    direction = -1;
                }

                rail = (rail as isize + direction) as usize;

                fence
            },
        );

        // Fold over the fence, filling out the resulting
        // string one position at a time as we process each rail.
        let mut chars = cipher.chars();
        fence
            .iter()
            .fold(vec!['?'; cipher_len], |mut result, rail| {
                for pos in rail.iter().filter_map(|&x| x) {
                    if let Some(ch) = chars.next() {
                        result[pos] = ch;
                    }
                }

                result
            })
            .iter()
            .collect()
    }
}
