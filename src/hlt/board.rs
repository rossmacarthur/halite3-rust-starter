use std::ops::{Add, Index, IndexMut, Sub};

use super::{Direction, DropoffId, Result, ShipId, ShipyardId};

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

impl Offset {
    /// Return the length of this Offset.
    pub fn len(&self) -> usize {
        (self.dx.abs() + self.dy.abs()) as usize
    }

    /// Return the unit direction for this Offset.
    pub fn signum(self) -> Offset {
        Offset {
            dx: self.dx.signum(),
            dy: self.dy.signum(),
        }
    }

    /// Return an Offset with only the X dimension of this Offset.
    pub fn dx(self) -> Offset {
        Offset { dx: self.dx, dy: 0 }
    }

    /// Return an Offset with only the Y dimension of this Offset.
    pub fn dy(self) -> Offset {
        Offset { dx: 0, dy: self.dy }
    }

    /// Return an Offset with the X dimension mirrored.
    pub fn mirrored_dx(self) -> Offset {
        Offset {
            dx: -self.dx,
            dy: self.dy,
        }
    }

    /// Return an Offset with the Y dimension mirrored.
    pub fn mirrored_dy(self) -> Offset {
        Offset {
            dx: self.dx,
            dy: -self.dy,
        }
    }

    /// Return an Offset with the X inverted around the given dimension.
    pub fn inverted_dx(self, width: isize) -> Offset {
        Offset {
            dx: invert(self.dx, width),
            dy: self.dy,
        }
    }

    /// Reduce an Offset to the smallest possible version, taking into account dimensions.
    pub fn reduce(self, width: isize, height: isize) -> Offset {
        *[
            self.inverted(width, height),
            self.inverted_dx(width),
            self.inverted_dy(height),
            self,
        ]
            .iter()
            .min_by_key(|o| o.len())
            .unwrap()
    }

    /// Return a Direction for this Offset.
    fn into_direction(self) -> Result<Direction> {
        let offset = self.signum();
        match (offset.dx, offset.dy) {
            (0, -1) => Ok(Direction::North),
            (1, 0) => Ok(Direction::East),
            (0, 1) => Ok(Direction::South),
            (-1, 0) => Ok(Direction::West),
            _ => Err(format_err!("unable to convert {:?} to Direction", self)),
        }
    }

    /// Return an Offset with the Y inverted around the given dimension.
    pub fn inverted_dy(self, height: isize) -> Offset {
        Offset {
            dx: self.dx,
            dy: invert(self.dy, height),
        }
    }

    /// Return an Offset inverted on both axis around the given dimensions.
    pub fn inverted(self, width: isize, height: isize) -> Offset {
        Offset {
            dx: invert(self.dx, width),
            dy: invert(self.dy, height),
        }
    }
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
    /// Return a new Position normalized to the given dimensions.
    pub fn normalized(self, width: isize, height: isize) -> Position {
        Position {
            x: normalize(self.x, width),
            y: normalize(self.y, height),
        }
    }

    /// Return the 4 adjacent Positions to the current Position.
    pub fn surrounding(&self) -> Vec<Position> {
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
}

/// Allow indexing the Board with Positions.
impl Index<Position> for Board {
    type Output = Cell;

    fn index(&self, index: Position) -> &Self::Output {
        let normalized = index.normalized(self.width, self.height);
        &self.cells[normalized.y as usize][normalized.x as usize]
    }
}

/// Allow mutably indexing the Board with Positions.
impl IndexMut<Position> for Board {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        let normalized = index.normalized(self.width, self.height);
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
    fn test_position_plus_offset() {
        let position = Position::new(2, 3);
        let offset = Offset::new(3, 4);
        assert_eq!(position + offset, Position::new(5, 7));

        let position = Position::new(2, 3);
        let offset = Offset::new(3, -4);
        assert_eq!(position + offset, Position::new(5, -1));
    }

    #[test]
    fn test_position_minus_position() {
        let position_a = Position::new(2, 3);
        let position_b = Position::new(5, 7);
        assert_eq!(position_b - position_a, Offset::new(3, 4));

        let position_a = Position::new(2, 3);
        let position_b = Position::new(5, -1);
        assert_eq!(position_b - position_a, Offset::new(3, -4));
    }

    #[test]
    fn test_position_normalized() {
        let width = 5;
        let height = 10;

        let input = Position::new(0, 0);
        assert_eq!(input.normalized(width, height), input);

        let input = Position::new(4, 9);
        assert_eq!(input.normalized(width, height), input);

        let input = Position::new(-3, 12);
        let output = Position::new(2, 2);
        assert_eq!(input.normalized(width, height), output);

        let input = Position::new(12, -3);
        let output = Position::new(2, 7);
        assert_eq!(input.normalized(width, height), output);

        let input = Position::new(-3, -15);
        let output = Position::new(2, 5);
        assert_eq!(input.normalized(width, height), output);
    }

    #[test]
    fn test_offset_inverted_dx() {
        let width = 5;

        let input = Offset::new(0, 0);
        assert_eq!(input.inverted_dx(width), input);

        let input = Offset::new(1, 1);
        assert_eq!(input.inverted_dx(width), Offset::new(-4, 1));

        let input = Offset::new(3, 4);
        assert_eq!(input.inverted_dx(width), Offset::new(-2, 4));
    }

    #[test]
    fn test_offset_inverted_dy() {
        let height = 10;

        let input = Offset::new(0, 0);
        assert_eq!(input.inverted_dy(height), input);

        let input = Offset::new(1, 1);
        assert_eq!(input.inverted_dy(height), Offset::new(1, -9));

        let input = Offset::new(3, 4);
        assert_eq!(input.inverted_dy(height), Offset::new(3, -6));
    }

    #[test]
    fn test_offset_inverted() {
        let width = 5;
        let height = 10;

        let input = Offset::new(0, 0);
        assert_eq!(input.inverted(width, height), input);

        let input = Offset::new(0, 2);
        let output = Offset::new(0, -8);
        assert_eq!(input.inverted(width, height), output);

        let input = Offset::new(2, 0);
        let output = Offset::new(-3, 0);
        assert_eq!(input.inverted(width, height), output);

        let input = Offset::new(3, 1);
        let output = Offset::new(-2, -9);
        assert_eq!(input.inverted(width, height), output);
    }

    #[test]
    fn test_offset_mirrored_dx() {
        let input = Offset::new(0, 0);
        assert_eq!(input.mirrored_dx(), input);

        let input = Offset::new(1, 1);
        assert_eq!(input.mirrored_dx(), Offset::new(-1, 1));

        let input = Offset::new(3, 4);
        assert_eq!(input.mirrored_dx(), Offset::new(-3, 4));
    }

    #[test]
    fn test_offset_mirrored_dy() {
        let input = Offset::new(0, 0);
        assert_eq!(input.mirrored_dy(), input);

        let input = Offset::new(1, 1);
        assert_eq!(input.mirrored_dy(), Offset::new(1, -1));

        let input = Offset::new(3, 4);
        assert_eq!(input.mirrored_dy(), Offset::new(3, -4));
    }

    #[test]
    fn test_offset_reduce() {
        let width = 5;
        let height = 5;

        let input = Offset::new(0, 0);
        assert_eq!(input.reduce(width, height), input);

        let input = Position::new(3, 2) - Position::new(0, 1);
        let output = Offset::new(-2, 1);
        assert_eq!(input.reduce(width, height), output);

        let input = Position::new(0, 1) - Position::new(3, 2);
        let output = Offset::new(2, -1);
        assert_eq!(input.reduce(width, height), output);

        let input = Position::new(0, 1) - Position::new(3, 2);
        let output = Offset::new(2, -1);
        assert_eq!(input.reduce(width, height), output);

        let input = Position::new(4, 4) - Position::new(0, 0);
        let output = Offset::new(-1, -1);
        assert_eq!(input.reduce(width, height), output);
    }
}
