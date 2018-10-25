# rust-halite3-starter

This is an alternate Rust starter kit to the one provided in [Halite III].

Additional features include:

- A lot more documentation.
- Use the [log] and [simplelog] crate so that the `debug!`, `info!` macros can
  be used for logging.
- Allow adding of Direction and Offset structs to Position structs. Subtracting
  two Positions returns an Offset.
- Derive Display/Into/From traits for ID new types so that they can be used like
  usizes.
- Add CLI for renaming the bot, overriding the log filename, and
  enabling/disabling logging.
- Use the failure crate for better error handling.
- Collect *all* constants from the Halite game engine.
- More idiomatic interaction with the Halite game engine.
- Improved API (in my opinion).

### CLI options

```
my_bot 0.1.0

My Halite III bot. See https://halite.io.

USAGE:
    my_bot [FLAGS] [OPTIONS]

FLAGS:
    -d, --debug      Whether to enable logging
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --log-file <filename>    Override the name of the log file
    -n, --name <name>            Override the name of the bot
```

[Halite III]: https://github.com/HaliteChallenge/Halite-III
[log]: https://github.com/rust-lang-nursery/log
[simplelog]: https://github.com/drakulix/simplelog.rs
