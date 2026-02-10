mod cli;
mod finder;
mod shell;

use clap::Parser;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Init { shell }) => {
            match shell::init_script(&shell) {
                Ok(script) => print!("{script}"),
                Err(e) => {
                    eprintln!("naive: {e}");
                    std::process::exit(1);
                }
            }
        }
        None => {
            match finder::pick() {
                Some(line) => {
                    let cmd = finder::extract_command(&line);
                    print!("{cmd}");
                }
                None => {
                    // User cancelled (Esc) — print nothing
                }
            }
        }
    }
}
