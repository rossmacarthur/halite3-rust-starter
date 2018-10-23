pub mod board;
pub mod constants;
pub mod engine;
pub mod util;

use std::collections::HashMap;

pub use self::board::{Board, Position};
use self::engine::Engine;
pub use self::util::Result;

/// A Player identifier.
#[derive(
    Clone, Constructor, Copy, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
pub struct PlayerId(usize);

/// A Player in the Game.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Player {
    /// This Player's identifier.
    pub id: PlayerId,
    /// The corresponding Shipyard for this Player.
    pub shipyard: Shipyard,
    /// A vector of owned Ships for this Player.
    pub ship_ids: Vec<ShipId>,
    /// A vector of owned Dropoffs for this Player.
    pub dropoff_ids: Vec<DropoffId>,
    /// The amount of halite this Player currently has.
    pub halite: usize,
}

impl Player {
    /// Create a new Player from a PlayerId and a Shipyard.
    pub fn new(id: PlayerId, shipyard: Shipyard) -> Self {
        Player {
            id,
            shipyard,
            ship_ids: Vec::new(),
            dropoff_ids: Vec::new(),
            halite: 0,
        }
    }
}

/// A Dropoff identifier.
#[derive(
    Clone, Constructor, Copy, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
pub struct DropoffId(usize);

/// A Dropoff in the Game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Dropoff {
    /// This Dropoff's identifier.
    pub id: DropoffId,
    /// Which player this Dropoff belongs to.
    pub player_id: PlayerId,
    /// The location of the Dropoff.
    pub position: Position,
}

/// A Shipyard identifier.
#[derive(
    Clone, Constructor, Copy, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
pub struct ShipyardId(usize);

// A Shipyard in the Game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Shipyard {
    /// This Shipyard's identifier.
    pub id: ShipyardId,
    /// Which player this Shipyard belongs to.
    pub player_id: PlayerId,
    /// The location of the Shipyard.
    pub position: Position,
}

/// A Ship identifier.
#[derive(
    Clone, Constructor, Copy, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd,
)]
pub struct ShipId(usize);

/// A ship in the Game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Ship {
    /// This Ship's identifier.
    pub id: ShipId,
    /// Which player this Ship belongs to.
    pub player_id: PlayerId,
    /// The location of the Ship.
    pub position: Position,
    /// The amount of halite the Ship currently has.
    pub halite: usize,
}

impl Ship {
    /// Create a new Ship.
    pub fn new(id: ShipId, player_id: PlayerId, position: Position, halite: usize) -> Ship {
        Ship {
            id,
            player_id,
            position,
            halite,
        }
    }

    /// Return whether the Ship has reached max halite carrying capacity.
    pub fn is_full(&self) -> bool {
        self.halite >= constants::get().max_halite
    }
}

/// A direction a Ship can take.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Return all the cardinals.
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
}

/// An action that a Ship can make.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Action {
    /// Stay still and collect ~25% of the halite at the location.
    Collect,
    /// Move 1 unit in the given direction.
    Move(Direction),
}

/// A command that can be given to the Halite engine.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Command {
    /// Spawn a new Ship!
    Spawn,
    /// Convert the given Ship to a Dropoff.
    ToDropoff(ShipId),
    /// Perform the given Action for the given Ship.
    Action(ShipId, Action),
}

/// The core Game struct.
#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    /// The current bot's identifier.
    pub my_id: PlayerId,
    /// The Board layout with locations of everything.
    pub board: Board,
    /// All the Players in this Game.
    pub players: HashMap<PlayerId, Player>,
    /// All the Ships in this Game.
    pub ships: HashMap<ShipId, Ship>,
    /// All the Dropoffs in this Game.
    pub dropoffs: HashMap<DropoffId, Dropoff>,
    /// Commands that will be sent when `end_turn` is called.
    pub commands: Vec<Command>,
    /// The current turn.
    pub turn: usize,
}

impl Game {
    /// Create a new Game from a PlayerId, Board, and Players.
    pub fn new(my_id: PlayerId, players: HashMap<PlayerId, Player>, board: Board) -> Self {
        Game {
            my_id,
            board,
            players,
            ships: HashMap::new(),
            dropoffs: HashMap::new(),
            commands: Vec::new(),
            turn: 0,
        }
    }

    /// Return our Player.
    pub fn me(&self) -> &Player {
        &self.players[&self.my_id]
    }

    /// Start a new Game.
    pub fn start() -> Result<Self> {
        let mut engine = Engine::new();
        constants::set(engine.recv()?);
        Ok(engine.recv()?)
    }

    /// Let the Halite engine know that we are ready to start playing.
    pub fn ready(&self, name: &str) {
        let engine = Engine::new();
        engine.send(name);
        engine.flush();
    }

    /// Update the Game information from the Halite engine.
    pub fn update(&mut self) -> Result<()> {
        Engine::new().update(self)?;
        info!("=============== TURN {} ================", self.turn);
        Ok(())
    }

    /// Spawn a Ship at the Shipyard.
    ///
    /// This adds the Ship to the Board, so that we can use it when considering collisions.
    pub fn spawn_ship(&mut self) {
        let id = if let Some(ship_id) = self.ships.keys().max() {
            ShipId::new(ship_id.0 + 1)
        } else {
            ShipId::new(0)
        };
        let position = self.me().shipyard.position;
        let ship = Ship::new(id, self.my_id, position, 0);
        self.board.add_ship(&ship);
        self.ships.insert(ship.id, ship);
        self.commands.push(Command::Spawn);
    }

    /// Move a Ship in the given Direction.
    pub fn move_ship(&mut self, ship: &mut Ship, direction: Direction) {
        self.board.move_ship(ship, direction);
        let command = Command::Action(ship.id, Action::Move(direction));
        self.commands.push(command);
    }

    /// Make a Ship collect halite in its current location.
    pub fn collect_halite(&mut self, ship: &Ship) {
        let command = Command::Action(ship.id, Action::Collect);
        self.commands.push(command);
    }

    /// End the turn and submit the commands.
    pub fn end_turn(&self) {
        let engine = Engine::new();
        for command in &self.commands {
            engine.send(command);
        }
        engine.flush();
    }
}
