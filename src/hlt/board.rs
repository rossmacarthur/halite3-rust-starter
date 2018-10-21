use std::ops::{Add, Index, IndexMut, Sub};

use super::{Direction, DropoffId, Ship, ShipId, ShipyardId};

/// Normalize a value to the given dimension.
///
/// This is the euclidean modulo operation.
fn normalize(value: isize, dimension: isize) -> isize {
    ((value % dimension) + dimension) % dimension
}

/// Invert a value around the given dimension.
///
/// The result is always the opposite sign to the given value.
fn invert(value: isize, dimension: isize) -> isize {
    let value = value % dimension;
    value + -value.signum() * dimension
}

/// A point on the Board.
#[derive(Clone, Constructor, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

// An offset between Positions.
#[derive(Clone, Constructor, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Offset {
    pub dx: isize,
    pub dy: isize,
}

/// A Direction can be easily converted to an Offset.
impl From<Direction> for Offset {
    fn from(direction: Direction) -> Offset {
        let (dx, dy) = match direction {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        };
        Offset::new(dx, dy)
    }
}

/// Support adding an Offset to a Position.
impl Add<Offset> for Position {
    type Output = Position;

    fn add(self, offset: Offset) -> Self::Output {
        Position::new(self.x + offset.dx, self.y + offset.dy)
    }
}

/// Support adding a Direction to a Position.
impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, direction: Direction) -> Self::Output {
        self + Offset::from(direction)
    }
}

/// Support subtracting Positions to get an Offset.
impl Sub<Position> for Position {
    type Output = Offset;

    fn sub(self, other: Position) -> Offset {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        Offset { dx, dy }
    }
}

impl Position {
    /// Return the 4 adjacent Positions to the current Position.
    pub fn get_surrounding(&self) -> Vec<Position> {
        vec![
            *self + Direction::North,
            *self + Direction::East,
            *self + Direction::South,
            *self + Direction::West,
        ]
    }
}

/// A simple wrapper for something that is either a Shipyard or a Dropoff.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Structure {
    Shipyard(ShipyardId),
    Dropoff(DropoffId),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Cell {
    /// The position of this Cell.
    pub position: Position,
    /// A structure that could be present in this Cell.
    pub structure: Option<Structure>,
    /// A ship that could be present in this Cell.
    pub ship: Option<ShipId>,
    /// The amount of halite in this Cell.
    pub halite: usize,
}

impl Cell {
    /// Create a new Cell.
    pub fn new(position: Position, halite: usize) -> Self {
        Cell {
            position,
            structure: None,
            ship: None,
            halite,
        }
    }

    /// Whether this Cell has a Ship.
    pub fn is_occupied(&self) -> bool {
        self.ship.is_some()
    }

    /// Whether this Cell has a Shipyard or Dropoff.
    pub fn has_structure(&self) -> bool {
        self.structure.is_some()
    }

    /// Whether this Cell has no Ship, Shipyard, or Dropoff.
    pub fn is_empty(&self) -> bool {
        !self.is_occupied() && !self.has_structure()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Board {
    /// The width of the Board.
    pub width: isize,
    /// The height of the Board.
    pub height: isize,
    /// A list of list of Cells for each Position on the Board.
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    /// Create a new empty Board.
    pub fn new(width: isize, height: isize) -> Board {
        let mut cells = Vec::with_capacity(height as usize);
        for y in 0..height {
            let mut row = Vec::with_capacity(width as usize);
            for x in 0..width {
                row.push(Cell::new(Position::new(x, y), 0));
            }
            cells.push(row);
        }
        Board {
            width,
            height,
            cells,
        }
    }

    /// Return a new Position normalized to the current Board.
    fn normalize(&self, position: Position) -> Position {
        Position {
            x: normalize(position.x, self.width),
            y: normalize(position.y, self.height),
        }
    }

    /// Return a new Offset, with a wrapped X according to the current Board.
    fn invert_x(&self, offset: Offset) -> Offset {
        Offset {
            dx: invert(offset.dx, self.width),
            dy: offset.dy,
        }
    }

    /// Return a new Offset, with a wrapped Y according to the current Board.
    fn invert_y(&self, offset: Offset) -> Offset {
        Offset {
            dx: offset.dx,
            dy: invert(offset.dy, self.height),
        }
    }

    /// Return a new Offset, with both X and Y inverted according to the current Board.
    fn invert(&self, offset: Offset) -> Offset {
        self.invert_y(self.invert_x(offset))
    }

    /// Return the smallest version of an Offset.
    /// This takes into account each wrapping possibility.
    fn smallest(&self, offset: Offset) -> Offset {
        *[
            self.invert(offset),
            self.invert_x(offset),
            self.invert_y(offset),
            offset,
        ]
            .iter()
            .min_by_key(|o| o.dx.abs() + o.dy.abs())
            .unwrap()
    }

    /// Return the best direction for the given Ship to move to.
    ///
    /// Naively goes in the Direction of the most halite.
    pub fn navigate_to_halite(&self, ship: &Ship) -> Option<Direction> {
        let directions = Direction::all();
        let mut cells: Vec<_> = directions
            .iter()
            .map(|d| (d, self[ship.position + *d]))
            .collect();
        cells.sort_by(|a, b| b.1.halite.cmp(&a.1.halite));

        for (direction, cell) in cells {
            if !cell.is_occupied() {
                return Some(*direction);
            }
        }

        None
    }
}

/// Allow indexing the Board with Positions.
impl Index<Position> for Board {
    type Output = Cell;

    fn index<'a>(&'a self, index: Position) -> &'a Self::Output {
        let normalized = self.normalize(index);
        &self.cells[normalized.y as usize][normalized.x as usize]
    }
}

/// Allow indexing the Board with Positions.
impl IndexMut<Position> for Board {
    fn index_mut<'a>(&'a mut self, index: Position) -> &'a mut Self::Output {
        let normalized = self.normalize(index);
        &mut self.cells[normalized.y as usize][normalized.x as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(normalize(-5, 10), 5);
        assert_eq!(normalize(5, 10), 5);
        assert_eq!(normalize(10, 10), 0);
    }

    #[test]
    fn test_invert() {
        assert_eq!(invert(-12, 10), 8);
        assert_eq!(invert(-10, 10), 0);
        assert_eq!(invert(-9, 10), 1);
        assert_eq!(invert(-5, 10), 5);
        assert_eq!(invert(0, 10), 0);
        assert_eq!(invert(1, 10), -9);
        assert_eq!(invert(5, 10), -5);
        assert_eq!(invert(10, 10), 0);
        assert_eq!(invert(19, 10), -1);
    }

    #[test]
    fn test_board_normalize() {
        let board = Board::new(5, 10);

        let input = Position::new(0, 0);
        assert_eq!(board.normalize(input), input);

        let input = Position::new(4, 9);
        assert_eq!(board.normalize(input), input);

        let input = Position::new(-3, 12);
        let output = Position::new(2, 2);
        assert_eq!(board.normalize(input), output);

        let input = Position::new(12, -3);
        let output = Position::new(2, 7);
        assert_eq!(board.normalize(input), output);

        let input = Position::new(-3, -15);
        let output = Position::new(2, 5);
        assert_eq!(board.normalize(input), output);
    }

    #[test]
    fn test_board_invert_x() {
        let board = Board::new(5, 10);

        let input = Offset::new(0, 0);
        assert_eq!(board.invert_x(input), input);

        let input = Offset::new(1, 1);
        assert_eq!(board.invert_x(input), Offset::new(-4, 1));

        let input = Offset::new(3, 4);
        assert_eq!(board.invert_x(input), Offset::new(-2, 4));
    }

    #[test]
    fn test_board_invert_y() {
        let board = Board::new(5, 10);

        let input = Offset::new(0, 0);
        assert_eq!(board.invert_y(input), input);

        let input = Offset::new(1, 1);
        assert_eq!(board.invert_y(input), Offset::new(1, -9));

        let input = Offset::new(3, 4);
        assert_eq!(board.invert_y(input), Offset::new(3, -6));
    }

    #[test]
    fn test_board_invert() {
        let board = Board::new(5, 10);

        let input = Offset::new(0, 0);
        assert_eq!(board.invert(input), input);

        let input = Offset::new(0, 2);
        let output = Offset::new(0, -8);
        assert_eq!(board.invert(input), output);

        let input = Offset::new(2, 0);
        let output = Offset::new(-3, 0);
        assert_eq!(board.invert(input), output);

        let input = Offset::new(3, 1);
        let output = Offset::new(-2, -9);
        assert_eq!(board.invert(input), output);
    }

    #[test]
    fn test_board_smallest() {
        let board = Board::new(5, 5);

        let input = Offset::new(0, 0);
        assert_eq!(board.smallest(input), input);

        let input = Position::new(3, 2) - Position::new(0, 1);
        let output = Offset::new(-2, 1);
        assert_eq!(board.smallest(input), output);

        let input = Position::new(0, 1) - Position::new(3, 2);
        let output = Offset::new(2, -1);
        assert_eq!(board.smallest(input), output);

        let input = Position::new(0, 1) - Position::new(3, 2);
        let output = Offset::new(2, -1);
        assert_eq!(board.smallest(input), output);

        let input = Position::new(4, 4) - Position::new(0, 0);
        let output = Offset::new(-1, -1);
        assert_eq!(board.smallest(input), output);
    }
}
