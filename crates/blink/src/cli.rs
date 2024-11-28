use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CLI {
    #[command(subcommand)]
    pub commands: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    /// Test for development.
    Test
}
