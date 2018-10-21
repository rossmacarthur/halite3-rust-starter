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
- Use the failure crate for better error handling.
- Collect *all* constants from the Halite game engine.
- More idiomatic interaction with the Halite game engine.
- Improved API (in my opinion).

[Halite III]: https://github.com/HaliteChallenge/Halite-III
[log]: https://github.com/rust-lang-nursery/log
[simplelog]: https://github.com/drakulix/simplelog.rs
