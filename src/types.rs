#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
    pub fn next_block(&self, x: usize, y: usize) -> (usize, usize) {
        let (mut temp_x, mut temp_y) = match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        };
        temp_x = x as isize + temp_x;
        temp_y = y as isize + temp_y;
        temp_x = if temp_x > 15 {
            temp_x - 16
        } else if temp_x < 0 {
            temp_x + 16
        } else {
            temp_x
        };
        temp_y = if temp_y > 15 {
            temp_y - 16
        } else if temp_y < 0 {
            temp_y + 16
        } else {
            temp_y
        };
        (temp_x as usize, temp_y as usize)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum SnakePart {
    Body,
    Head,
    Tail,
}

#[derive(Copy, Clone, PartialEq)]
pub enum CellState {
    Empty,
    Apple,
    Snake(SnakePart, Direction),
}

#[derive(Copy, Clone, PartialEq)]
pub enum GameState {
    Playing,
    Paused,
    GameOver,
}

pub type CellData = [[CellState; 16]; 16];
