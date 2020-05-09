// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { position: (x,y), direction: d}
    }

    pub fn turn_right(self) -> Self {
        Self {position: self.position, direction: 
            match self.direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North
            }
        }
    }

    pub fn turn_left(self) -> Self {
        Self {position: self.position, direction: 
            match self.direction {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North
            }
        }
    }

    pub fn advance(self) -> Self {
        Self { position: 
            match self.direction {
                Direction::North => (self.position.0, self.position.1 + 1),
                Direction::East => (self.position.0 + 1, self.position.1),
                Direction::South => (self.position.0, self.position.1 - 1),
                Direction::West => (self.position.0 - 1, self.position.1)
            },
            direction: self.direction
        }
        
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |acc, c| {
            match c {
                'A' => acc.advance(),
                'R' => acc.turn_right(),
                'L' => acc.turn_left(),
                _ => panic!()
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
