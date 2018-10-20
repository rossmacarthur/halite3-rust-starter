use std::ops::{Index, IndexMut};

use super::{Direction, DropoffId, ShipId, ShipyardId};

/// A point on the Board.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    /// Return a new Position normalized to the given Board.
    pub fn normalized(&self, board: &Board) -> Position {
        let width = board.width as isize;
        let height = board.height as isize;
        let x = ((self.x % width) + width) % width;
        let y = ((self.y % height) + height) % height;
        Position { x, y }
    }

    /// Return a new Position offsetted by the given Direction.
    pub fn offset(&self, d: Direction) -> Position {
        let (dx, dy) = match d {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        };
        Position {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    /// Return the 4 adjacent Positions to the current Position.
    pub fn get_surrounding(&self) -> Vec<Position> {
        vec![
            self.offset(Direction::North),
            self.offset(Direction::East),
            self.offset(Direction::South),
            self.offset(Direction::West),
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

    /// Whether this Cell has no Ship, Shipyard, or Dropoff.
    pub fn is_empty(&self) -> bool {
        self.ship.is_none() && self.structure.is_none()
    }

    /// Whether this Cell has a Ship.
    pub fn is_occupied(&self) -> bool {
        self.ship.is_some()
    }

    /// Whether this Cell has a Shipyard or Dropoff.
    pub fn has_structure(&self) -> bool {
        self.structure.is_some()
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

/// Allow indexing the Board with Positions.
impl Index<Position> for Board {
    type Output = Cell;

    fn index<'a>(&'a self, index: Position) -> &'a Self::Output {
        let normalized = index.normalized(self);
        &self.cells[normalized.y as usize][normalized.x as usize]
    }
}

/// Allow indexing the Board with Positions.
impl IndexMut<Position> for Board {
    fn index_mut<'a>(&'a mut self, index: Position) -> &'a mut Self::Output {
        let normalized = index.normalized(self);
        &mut self.cells[normalized.y as usize][normalized.x as usize]
    }
}
