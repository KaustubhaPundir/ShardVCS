use clap::{Parser, Subcommand};

mod repo;
mod object;
mod index;
mod commands;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init,
    Add { path: String },
    Status,
    Commit { message: String },
    Log,
    Checkout { hash: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Init => commands::init::run(),
        Command::Add { path } => commands::add::run(&path),
        Command::Status => commands::status::run(),
        Command::Commit { message } => commands::commit::run(&message),
        Command::Log => commands::log::run(),
        Command::Checkout { hash } => commands::checkout::run(&hash),
    }
}
