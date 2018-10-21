mod recv;
mod send;

use std::error::Error;
use std::fmt;
use std::io;
use std::str;

use super::Result;

use self::recv::FromEngine;
use self::send::ToEngine;

/// An Error that can occur when parsing.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EngineParseError;

/// Implement display so that it works with the failure crate.
impl fmt::Display for EngineParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.description())
    }
}

/// Add a description so that it looks nice when pretty printed.
impl Error for EngineParseError {
    fn description(&self) -> &str {
        "unable to parse data from engine"
    }
}

/// Struct to handle input and output to the Halite game engine.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Engine {
    tokens: Vec<String>,
}

impl Engine {
    /// Create a new Engine object.
    pub fn new() -> Self {
        Engine { tokens: Vec::new() }
    }

    /// Read a single line from stdin.
    pub fn next_line(&self) -> Result<String> {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        Ok(buffer)
    }

    /// Read an arbitrary thing from stdin, as long as it implements FromStr.
    pub fn next<T: str::FromStr>(&mut self) -> Result<T> {
        while self.tokens.len() == 0 {
            let buffer = self.next_line()?;
            let tokens = buffer.split_whitespace().map(|s| s.to_string());
            self.tokens.extend(tokens);
        }
        let token = &self.tokens.remove(0);
        Ok(T::from_str(token).map_err(|_| EngineParseError)?)
    }

    /// Print an arbitrary thing to stdout, as long as it implements Display.
    pub fn print<T: fmt::Display>(&self, obj: T) {
        print!("{} ", obj);
    }

    /// Read an arbitrary thing from stdin, as long as it implements FromEngine.
    pub fn recv<T: FromEngine>(&mut self) -> Result<T> {
        T::new_from_engine(self)
    }

    /// Update an arbitrary thing from stdin, as long as it implements FromEngine.
    pub fn update<T: FromEngine>(&mut self, obj: &mut T) -> Result<()> {
        obj.update_from_engine(self)
    }

    /// Write something to stdout, as long as it implements ToEngine.
    pub fn send<T: ToEngine>(&self, obj: T) {
        obj.send_to_engine(self)
    }

    /// Flush the stdout.
    pub fn flush(&self) {
        self.send("\n");
    }
}

impl Drop for Engine {
    /// If the Engine still has tokens we want to panic, because something went wrong.
    fn drop(&mut self) {
        assert!(self.tokens.is_empty());
    }
}
