pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        if self.0 == 0 {
            return Vec::new();
        }

        (0..self.0).fold(vec![vec![1]], |mut rows, row_idx| {
            if row_idx == 0 {
                return rows;
            }

            let new_row = (0..=row_idx)
                .map(|col_idx| {
                    let prev_row = &rows[(row_idx - 1) as usize];
                    match col_idx {
                        0 => 1,
                        i if i == row_idx => 1,
                        i => prev_row[(i - 1) as usize] + prev_row[i as usize],
                    }
                })
                .collect();

            rows.push(new_row);
            rows
        })
    }
}
