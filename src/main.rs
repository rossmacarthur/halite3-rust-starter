#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate simplelog;

mod hlt;

use std::process;

use rand::Rng;

use hlt::util::{configure_logger, pretty_error, Result};
use hlt::*;

fn run() -> Result<()> {
    // The name of our bot.
    let name = "MyBot";

    // Start a new Game, by reading the game information from the Halite engine.
    let mut game = Game::start()?;

    // Configure the logger, so that we can use debug! and other log macros.
    // It will log to a file called "MyBot-<game-seed>-<my-id>.log"
    configure_logger(name, game.my_id)?;

    // At this point "game" variable is populated with initial map data.
    // This is a good place to do computationally expensive start-up pre-processing.
    // ...
    // ...
    // ...

    // This bot is a random bot, so we need a randomizer to pick the ship direction.
    let mut rng = rand::thread_rng();

    // Call "ready" function below, the 2 second per turn timer will start now.
    game.ready(name);
    info!(
        "Successfully initialized {}! Player ID is {}",
        name, game.my_id
    );

    loop {
        // Get the updated Game from the Halite engine.
        game.update()?;

        let me = &game.players[&game.my_id];

        // Loop through all of our Ships and basically randomly generate the direction.
        for ship_id in &me.ship_ids {
            let ship = game.ships[ship_id];
            let cell = game.board[ship.position];

            let command = if cell.halite < constants::get().max_halite / 10 || ship.is_full() {
                let random_direction = Direction::all()[rng.gen_range(0, 4)];
                Command::Action(ship.id, Action::Move(random_direction))
            } else {
                Command::Action(ship.id, Action::Collect)
            };

            game.commands.push(command);
        }

        let shipyard_cell = game.board[me.shipyard.position];

        // If we have enough halite, spawn a new ship!
        if game.turn <= 400
            && me.halite >= constants::get().new_entity_halite_cost
            && !shipyard_cell.is_occupied()
        {
            game.commands.push(Command::Spawn);
        }

        game.end_turn();
    }
}

fn main() {
    if let Err(ref e) = run() {
        let pretty = pretty_error(e);
        error!("Fatal error: {}", pretty);
        eprintln!("Fatal error: {}", pretty);
        process::exit(1);
    }
}
