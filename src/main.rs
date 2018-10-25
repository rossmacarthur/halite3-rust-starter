#[macro_use]
extern crate clap;
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

use clap::{App, Arg};
use rand::Rng;

use hlt::util::{configure_logger, pretty_error, Result};
use hlt::*;

fn run() -> Result<()> {
    // Parse command line arguments.
    let cli = App::new(crate_name!())
        .version(crate_version!())
        .about("\nMy Halite III bot. See https://halite.io.")
        .arg(
            Arg::with_name("debug")
                .short("-d")
                .long("--debug")
                .help("Whether to enable logging"),
        ).arg(
            Arg::with_name("filename")
                .short("-l")
                .long("--log-file")
                .takes_value(true)
                .help("Override the name of the log file"),
        ).arg(
            Arg::with_name("name")
                .short("-n")
                .long("--name")
                .takes_value(true)
                .help("Override the name of the bot"),
        ).get_matches();

    // The name of our bot.
    let default_name = format!("MyBot-{}", crate_version!());
    let name = cli.value_of("name").unwrap_or(&default_name);

    // Start a new Game, by reading the game information from the Halite engine.
    let mut game = Game::start()?;

    // Configure the logger, so that we can use debug! and other log macros.
    // It will log to a file called "MyBot-<game-seed>-<my-id>.log"
    if cli.is_present("debug") {
        let log_filename = format!("{}-{}-{}.log", name, constants::get().game_seed, game.my_id);
        configure_logger(cli.value_of("filename").unwrap_or(&log_filename))?;
    }

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
                Command::Move(ship.id, random_direction)
            } else {
                Command::Collect(ship.id)
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
