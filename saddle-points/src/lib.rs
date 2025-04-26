pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    if input.is_empty() || input[0].is_empty() {
        return points;
    }

    let rows = input.len();
    let cols = input[0].len();
    let column_minimums: Vec<u64> = (0..cols)
        .map(|col| (0..rows).map(|row| input[row][col]).min().unwrap())
        .collect();

    for (row_idx, row) in input.iter().enumerate() {
        let max_in_row = row.iter().max().unwrap();
        for (col_idx, candidate) in row.iter().enumerate() {
            if candidate == max_in_row && *candidate == column_minimums[col_idx] {
                points.push((row_idx, col_idx));
            }
        }
    }

    points
}
