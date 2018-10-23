use std::fmt;

use super::Engine;

use super::super::{Action, Command, Direction};

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
                Command::ToDropoff(ship_id) => format!("c {}", ship_id),
                Command::Action(ship_id, action) => {
                    let c = match action {
                        Action::Move(Direction::North) => 'n',
                        Action::Move(Direction::East) => 'e',
                        Action::Move(Direction::South) => 's',
                        Action::Move(Direction::West) => 'w',
                        Action::Collect => 'o',
                    };
                    format!("m {} {}", ship_id, c)
                }
            } + " ",
        )
    }
}
