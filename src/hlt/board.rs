use std::cmp::Ordering;
use std::ops::{Add, Index, IndexMut, Sub};

use super::{Direction, DropoffId, Ship, ShipId, ShipyardId};

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

impl Offset {
    /// Return the absolute value of this offset.
    pub fn abs(self) -> Offset {
        Offset {
            dx: self.dx.abs(),
            dy: self.dy.abs(),
        }
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
    pub width: usize,
    /// The height of the Board.
    pub height: usize,
    /// A list of list of Cells for each Position on the Board.
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    /// Return a new Position normalized to the given Board.
    pub fn normalize(&self, position: Position) -> Position {
        let width = self.width as isize;
        let height = self.height as isize;
        let x = ((position.x % width) + width) % width;
        let y = ((position.y % height) + height) % height;
        Position { x, y }
    }

    /// Return the best direction for the given Ship to move to.
    pub fn naive_navigate(&self, ship: &Ship) -> Option<Direction> {
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
