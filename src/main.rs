use anyhow::Result;
use clap::{Parser, Subcommand};
use human_panic::setup_panic;
use std::path::PathBuf;
mod data;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(name = "Chores")]
#[command(version = "1.0")]
#[command(about = "Managing household chores with ease")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Perform operation on a person
#[derive(Subcommand)]
enum PersonCommand {
    /// add a new person
    Add {
        /// name of the person to add
        #[arg(short, long)]
        name: String,
    },

    /// remove a person
    Remove {
        /// index of the person to remove
        #[arg(short, long)]
        index: u8,
    },
}

#[derive(Subcommand)]
enum Commands {
    Person {
        /// Operations on a person
        #[command(subcommand)]
        command: PersonCommand,
    },
    /// Adds a chore with description, level and frequency
    AddChore {
        /// chore description
        #[arg(short, long)]
        description: String,

        /// difficulty level
        #[arg(short, long)]
        level: u8,

        #[arg(short)]
        freq_days: u8,
    },
    /// Prints report for all persons, chores and assignments
    Report,
    /// Assigns a person to a chore
    Assign {
        /// whom to assing
        #[arg(short, long)]
        person: i32,
        /// what chore to assign
        #[arg(short, long)]
        chore: i32,
    },
}

fn dispatch(d: &mut impl data::Data) -> Result<(), data::DataError> {
    let args = Cli::parse();
    match &args.command {
        Commands::Person { command } => match command {
            PersonCommand::Add { name } => d.add_person(name),
            PersonCommand::Remove { index } => d.remove_person(*index),
        },
        Commands::AddChore {
            description,
            level,
            freq_days,
        } => d.add_chore(description, *level, *freq_days),
        Commands::Report => d.report(),
        Commands::Assign { person, chore } => d.assign(*person, *chore),
    }
}
fn main() -> Result<(), data::DataError> {
    setup_panic!();
    dispatch(&mut data::db(&PathBuf::from("./test.db")))
}
