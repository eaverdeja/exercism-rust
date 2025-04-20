pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() {
        return Vec::new();
    }

    let height = minefield.len();
    let width = minefield.first().map_or(0, |row| row.len());

    let matrix: Vec<Vec<u8>> = minefield
        .iter()
        .map(|row| row.as_bytes().to_vec())
        .collect();

    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| match matrix[y][x] {
                    b' ' => {
                        let count = count_neighbouring_mines(&matrix, y, x);
                        if count > 0 {
                            char::from_digit(count, 10).unwrap()
                        } else {
                            ' '
                        }
                    }
                    _ => '*',
                })
                .collect()
        })
        .collect()
}

fn count_neighbouring_mines(matrix: &[Vec<u8>], row_idx: usize, column_idx: usize) -> u32 {
    let height = matrix.len() as isize;
    let width = if height > 0 {
        matrix[0].len() as isize
    } else {
        0
    };
    let row_idx = row_idx as isize;
    let column_idx = column_idx as isize;

    let mut count = 0;

    // dy represents the distance in the y-axis
    for dy in -1..=1 {
        // dx represents the distance in the x-axis
        for dx in -1..=1 {
            // don't count the cell itself
            if dy == 0 && dx == 0 {
                continue;
            }

            // neighbouring coordiantes
            let (ny, nx) = (row_idx + dy, column_idx + dx);

            // are the neighbouring coordinates in bounds
            // and are a mine?
            if ny >= 0
                && ny < height
                && nx >= 0
                && nx < width
                && matrix[ny as usize][nx as usize] == b'*'
            {
                count += 1;
            }
        }
    }

    count
}
