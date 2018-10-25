use std::collections::HashMap;
use std::marker;
use std::str;

use serde_json;

use super::super::board::{Board, Position, Structure};
use super::super::constants::Constants;
use super::super::{
    Dropoff, DropoffId, Game, Player, PlayerId, Result, Ship, ShipId, Shipyard, ShipyardId,
};
use super::{Engine, EngineParseError};

/// A trait for creating and updating types from the Halite engine.
pub trait FromEngine
where
    Self: marker::Sized,
{
    /// Create this object from data from the Halite engine.
    fn new_from_engine(engine: &mut Engine) -> Result<Self>;

    /// Update this object with data from the Halite engine.
    /// By default this just returns the same object.
    fn update_from_engine(&mut self, _engine: &mut Engine) -> Result<()> {
        Ok(())
    }
}

impl<T> FromEngine for T
where
    T: str::FromStr,
{
    /// Anything that implements FromStr can be read easily from the Engine.
    fn new_from_engine(engine: &mut Engine) -> Result<Self> {
        engine.next()
    }
}

impl FromEngine for DropoffId {
    /// Read a usize and then convert into a DropoffId.
    fn new_from_engine(engine: &mut Engine) -> Result<Self> {
        Ok(engine.next::<usize>()?.into())
    }
}

impl FromEngine for PlayerId {
    /// Read a usize and then convert into a PlayerId.
    fn new_from_engine(engine: &mut Engine) -> Result<Self> {
        Ok(engine.next::<usize>()?.into())
    }
}

impl FromEngine for ShipId {
    /// Read a usize and then convert into a ShipId.
    fn new_from_engine(engine: &mut Engine) -> Result<Self> {
        Ok(engine.next::<usize>()?.into())
    }
}

impl FromEngine for ShipyardId {
    /// Read a usize and then convert into a ShipyardId.
    fn new_from_engine(engine: &mut Engine) -> Result<Self> {
        Ok(engine.next::<usize>()?.into())
    }
}

impl FromEngine for Constants {
    /// Read Constants given as a single line of JSON.
    fn new_from_engine(engine: &mut Engine) -> Result<Self> {
        Ok(serde_json::from_str(&engine.next_line()?)?)
    }
}

impl FromEngine for Position {
    /// Read an X Y point from the Engine.
    fn new_from_engine(engine: &mut Engine) -> Result<Self> {
        let x = engine.next()?;
        let y = engine.next()?;
        Ok(Position { x, y })
    }
}

impl FromEngine for Player {
    /// Read a Player from the Engine.
    fn new_from_engine(engine: &mut Engine) -> Result<Self> {
        let player_id: PlayerId = engine.recv()?;
        let position = engine.recv()?;
        let u: usize = player_id.into();
        let shipyard = Shipyard {
            id: ShipyardId::from(u),
            player_id,
            position,
        };
        Ok(Player::new(player_id, shipyard))
    }
}

impl FromEngine for Board {
    /// Read the entire game Board from the Engine.
    fn new_from_engine(engine: &mut Engine) -> Result<Self> {
        let width = engine.next()?;
        let height = engine.next()?;

        let mut board = Board::new(width, height);

        for y in 0..height as usize {
            for x in 0..width as usize {
                board.cells[y][x].halite = engine.next()?;
            }
        }

        Ok(board)
    }

    /// Update the data in the given Board from the Engine.
    fn update_from_engine(&mut self, engine: &mut Engine) -> Result<()> {
        // Clear all the ship locations.
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                self.cells[y][x].ship = None;
            }
        }

        // Read in the new halite values for each Cell.
        for _ in 0..engine.recv()? {
            let position = engine.recv()?;
            let halite = engine.recv()?;
            self[position].halite = halite;
        }

        Ok(())
    }
}

impl FromEngine for Game {
    /// Construct a new Game from the Engine.
    fn new_from_engine(engine: &mut Engine) -> Result<Self> {
        // Read the player count.
        let player_count = engine.recv()?;

        // Read our player identifier.
        let my_id = engine.recv()?;

        // Read in the Players.
        let mut players = HashMap::with_capacity(player_count);

        for _ in 0..player_count {
            let player: Player = engine.recv()?;
            players.insert(player.id, player);
        }

        // Read in the Board.
        let board = engine.recv()?;

        Ok(Game::new(my_id, players, board))
    }

    /// Update the Game frame from the Engine.
    fn update_from_engine(&mut self, engine: &mut Engine) -> Result<()> {
        self.turn = engine.recv()?;

        // Clone the old Ships and Dropoffs from the previous frame.
        // This is so we can keep state in Ships and Dropoffs if we so wish.
        let mut old_ships = self.ships.clone();
        let mut old_dropoffs = self.dropoffs.clone();

        // Clear all of these because we will reconstruct these.
        self.ships.clear();
        self.dropoffs.clear();
        self.commands.clear();

        for _ in 0..self.players.len() {
            // Read the player ID and get the corresponding Player.
            let player_id = engine.recv()?;
            let mut player = self.players.get_mut(&player_id).ok_or(EngineParseError)?;

            let ship_count = engine.recv()?;
            let dropoff_count = engine.recv()?;
            player.halite = engine.recv()?;

            // Update the Ships.
            player.ship_ids.clear();
            for _ in 0..ship_count {
                let id = engine.recv()?;
                let position = engine.recv()?;
                let halite = engine.recv()?;

                let ship = if let Some(ship) = old_ships.get_mut(&id) {
                    ship.position = position;
                    ship.halite = halite;
                    *ship
                } else {
                    Ship::new(id, player_id, position, halite)
                };

                self.ships.insert(id, ship);
                player.ship_ids.push(id);
            }

            // Update the Dropoffs.
            player.dropoff_ids.clear();
            for _ in 0..dropoff_count {
                let id = engine.recv()?;
                let position = engine.recv()?;

                let dropoff = if let Some(dropoff) = old_dropoffs.get_mut(&id) {
                    dropoff.position = position;
                    *dropoff
                } else {
                    Dropoff::new(id, player_id, position)
                };

                self.dropoffs.insert(id, dropoff);
                player.dropoff_ids.push(id);
            }
        }

        engine.update(&mut self.board)?;

        for player in self.players.values() {
            let shipyard = &player.shipyard;
            self.board[shipyard.position].structure = Some(Structure::Shipyard(shipyard.id));

            for ship_id in &player.ship_ids {
                let ship = &self.ships[ship_id];
                self.board[ship.position].ship = Some(*ship_id);
            }

            for dropoff_id in &player.dropoff_ids {
                let dropoff = &self.dropoffs[dropoff_id];
                self.board[dropoff.position].structure = Some(Structure::Dropoff(*dropoff_id));
            }
        }

        Ok(())
    }
}
