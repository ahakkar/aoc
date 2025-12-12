use crate::util::point2::Point2;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    Still,
}

impl Direction {
    pub const DIRS: [Direction; 8] = [
        Direction::NorthWest,
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
    ];

    pub const ORTHOGONAL: [Direction; 4] = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];

    pub const DIAGONAL: [Direction; 4] = [
        Direction::NorthEast,
        Direction::NorthWest,
        Direction::SouthEast,
        Direction::SouthWest,
    ];

    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::SouthWest => Direction::NorthEast,
            Direction::NorthWest => Direction::SouthEast,
            Direction::SouthEast => Direction::NorthWest,
            Direction::Still => Direction::Still,
        }
    }

    pub fn to_vector(self) -> Point2 {
        match self {
            // col row
            Direction::East => Point2::new(1, 0),
            Direction::North => Point2::new(0, -1),
            Direction::West => Point2::new(-1, 0),
            Direction::South => Point2::new(0, 1),
            Direction::NorthEast => Point2::new(1, -1),
            Direction::NorthWest => Point2::new(-1, -1),
            Direction::SouthEast => Point2::new(1, 1),
            Direction::SouthWest => Point2::new(-1, 1),
            Direction::Still => Point2::new(0, 0),
        }
    }

    pub fn turn_90(cur_dir: Direction, turn: char) -> Direction {
        match turn {
            'l' => match cur_dir {
                Direction::East => Direction::North,
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                _ => panic!("invalid dir"),
            },
            'r' => match cur_dir {
                Direction::East => Direction::South,
                Direction::North => Direction::East,
                Direction::West => Direction::North,
                Direction::South => Direction::West,
                _ => panic!("invalid dir"),
            },
            _ => panic!("Invalid turn"),
        }
    }
}
