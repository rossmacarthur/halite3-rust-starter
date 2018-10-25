use failure;
use simplelog;

use std::fs;

pub type Result<T> = ::std::result::Result<T, failure::Error>;

/// Configure the global logger.
pub fn configure_logger(filename: &str) -> Result<()> {
    // Setup a simple log file.
    let log_config = simplelog::Config {
        time: None,
        target: None,
        ..Default::default()
    };
    let log_file = fs::File::create(filename)?;
    let _ = simplelog::WriteLogger::init(simplelog::LevelFilter::Debug, log_config, log_file);

    Ok(())
}

/// Return a prettily formatted error, including its entire causal chain.
pub fn pretty_error(err: &failure::Error) -> String {
    let mut pretty = err.to_string();
    for e in err.iter_chain().skip(1) {
        pretty.push_str(&format!("\n     Due to: {}", e).to_owned());
    }
    pretty
}
