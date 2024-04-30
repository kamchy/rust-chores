use std::path::PathBuf;

use clap::{Parser, Subcommand};
use human_panic::setup_panic;
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

#[derive(Subcommand)]
enum Commands {
    /// adds a person
    AddPerson { name: String },
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

fn dispatch(d: &mut impl data::Data) {
    let args = Cli::parse();
    let _ = match &args.command {
        Commands::AddPerson { name } => d.add_person(name),
        Commands::AddChore {
            description,
            level,
            freq_days,
        } => d.add_chore(description, *level, *freq_days),
        Commands::Report => d.report().map(|_| 0),
        Commands::Assign { person, chore } => d.assign(*person, *chore),
    };
}
fn main() {
    setup_panic!();
    dispatch(&mut data::db(&PathBuf::from("./test.db")))
}
