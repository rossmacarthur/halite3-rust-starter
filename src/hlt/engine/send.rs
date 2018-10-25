use std::fmt;

use super::Engine;

use super::super::{Command, Direction};

/// A trait for sending types to the Halite engine.
pub trait ToEngine {
    /// Send this object to the engine.
    fn send_to_engine(&self, engine: &Engine);
}

impl<T> ToEngine for T
where
    T: fmt::Display,
{
    /// Automatically implement for types that implement Display.
    fn send_to_engine(&self, engine: &Engine) {
        engine.print(format!("{}", self))
    }
}

impl<'a> ToEngine for &'a Command {
    /// Send a Command to the engine.
    fn send_to_engine(&self, engine: &Engine) {
        engine.print(
            match self {
                Command::Spawn => format!("g"),
                Command::ConvertToDropoff(ship_id) => format!("c {}", ship_id),
                Command::Collect(ship_id) => format!("m {} o", ship_id),
                Command::Move(ship_id, direction) => format!(
                    "m {} {}",
                    ship_id,
                    match direction {
                        Direction::North => 'n',
                        Direction::East => 'e',
                        Direction::South => 's',
                        Direction::West => 'w',
                    }
                ),
            } + " ",
        )
    }
}
