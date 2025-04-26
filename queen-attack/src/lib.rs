const BOARD_SIZE: i32 = 8;

#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..BOARD_SIZE, 0..BOARD_SIZE) => Some(Self { rank, file }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let rd = self.position.rank.abs_diff(other.position.rank);
        let fd = self.position.file.abs_diff(other.position.file);

        // Same rank, same file or same diagonal
        rd == 0 || fd == 0 || rd == fd
    }
}
