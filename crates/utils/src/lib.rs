use std::fs::File;

use anyhow::Result;
use log::LevelFilter;
use simplelog::{CombinedLogger, Config, WriteLogger};

/// Initializes the logging system.
pub fn init_logging() -> Result<()> {
    let log_file = File::create("blink.log")?;

    // Configure the mf.
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        log_file,
    )])?;

    Ok(())
}

// Export the crates from logging because we only want to
// have to import the `utils` create, not `log` and `simplelog`.
//
// USAGE: info!("debug ma man");
pub use log::{debug, error, info, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VimMode {
    Normal,
    Insert, // Insert mode is the default when there vim_mode is false.
    Visual,
}
