pub struct PascalsTriangle(usize);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count as usize)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.0).fold(Vec::with_capacity(self.0), |mut rows, row_idx| {
            rows.push(self.row(&rows, row_idx));
            rows
        })
    }

    fn row(&self, rows: &[Vec<u32>], row_idx: usize) -> Vec<u32> {
        (0..=row_idx)
            .map(|col_idx| match col_idx {
                0 => 1,
                end if end == row_idx => 1,
                i => rows[row_idx - 1][i - 1] + rows[row_idx - 1][i],
            })
            .collect()
    }
}
