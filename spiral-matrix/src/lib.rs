use Direction::*;

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Cursors {
    top: usize,
    left: usize,
    right: usize,
    bottom: usize,
}

impl Cursors {
    fn new(size: usize) -> Self {
        Self {
            top: 0,
            left: 0,
            right: size - 1,
            bottom: size - 1,
        }
    }

    fn update(&mut self, direction: &Direction) {
        match direction {
            Right => self.top += 1,
            Down => self.right -= 1,
            Left => self.bottom -= 1,
            Up => self.left += 1,
        }
    }
}

impl Direction {
    fn next(self) -> Self {
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
        }
    }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return Vec::new();
    }

    let size = size as usize;
    let mut result = vec![vec![0; size]; size];
    let mut values = 1..=size.pow(2) as u32;
    let mut next_value = || values.next().expect("should not be empty!");

    let mut cursors = Cursors::new(size);
    let mut direction = Right;

    while {
        let Cursors {
            top,
            left,
            right,
            bottom,
        } = cursors;
        top <= bottom && left <= right
    } {
        let Cursors {
            top,
            left,
            right,
            bottom,
        } = cursors;

        match direction {
            Right => (left..=right).for_each(|i| result[top][i] = next_value()),
            Down => (top..=bottom).for_each(|i| result[i][right] = next_value()),
            Left => (left..=right)
                .rev()
                .for_each(|i| result[bottom][i] = next_value()),
            Up => (top..=bottom)
                .rev()
                .for_each(|i| result[i][left] = next_value()),
        }

        cursors.update(&direction);
        direction = direction.next()
    }

    result
}
