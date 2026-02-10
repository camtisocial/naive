use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "naive", version, about = "AI-assisted command snippet manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    Init {
        shell: String,
    },
}
