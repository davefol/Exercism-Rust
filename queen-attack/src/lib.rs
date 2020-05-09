#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (x, y) if x <= 7 && x >= 0 && y>=0 && y<=7 => Some(Self {rank, file}),
            _ => None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let rank_diff = self.position.rank - other.position.rank;
        let file_diff = self.position.file - other.position.file;
        rank_diff.abs() == file_diff.abs() || (rank_diff == 0 && file_diff != 0) || (rank_diff != 0 && file_diff == 0)

    }
}
