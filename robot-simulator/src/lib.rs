use Direction::*;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn right(&self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn left(&self) -> Self {
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let new_direction = self.direction.right();
        self.with_direction(new_direction)
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let new_direction = self.direction.left();
        self.with_direction(new_direction)
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = (self.x, self.y);
        let (dx, dy) = match self.direction {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        };

        self.with_coordinates(x + dx, y + dy)
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |robot, instruction| match instruction {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => robot,
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    fn with_direction(self, direction: Direction) -> Self {
        Self { direction, ..self }
    }

    fn with_coordinates(self, x: i32, y: i32) -> Self {
        Self { x, y, ..self }
    }
}
