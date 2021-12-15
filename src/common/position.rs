#[derive(Debug, Default, Eq, PartialEq, Hash, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    pub fn from_comma_separated_line(line: String) -> Position {
        let mut parts = line
            .split(",")
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();

        Position {
            x: parts.remove(0),
            y: parts.remove(0),
        }
    }

    pub fn adjacent(&self, max_x: usize, max_y: usize) -> Vec<Position> {
        let mut adjacent = Vec::new();

        if self.x > 0 {
            adjacent.push(Position::new(self.x - 1, self.y));
        }
        if self.y > 0 {
            adjacent.push(Position::new(self.x, self.y - 1));
        }
        if self.x < max_x - 1 {
            adjacent.push(Position::new(self.x + 1, self.y));
        }
        if self.y < max_y - 1 {
            adjacent.push(Position::new(self.x, self.y + 1));
        }

        adjacent
    }
}
