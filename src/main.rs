///! chores is a simple commandline application
///! for managing chores and assignments of chores
///! to people in my household.
use anyhow::Result;
use clap::{builder::PathBufValueParser, Parser, Subcommand};
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
    #[arg(short, long, default_value = "test.db")]
    dbpath: String,
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
    /// list all persons
    List,
}

#[derive(Subcommand)]
enum ChoreCommand {
    /// Adds a chore with description, level and frequency
    Add {
        /// chore description
        #[arg(short, long)]
        description: String,

        /// difficulty level
        #[arg(short, long)]
        level: u8,

        #[arg(short)]
        freq_days: u8,
    },
    /// Removes a chore by providing index
    Remove {
        /// index of the chore to remove
        #[arg(short, long)]
        index: u8,
    },
    /// List all chores
    List,
}
/// A command for managing assignments
#[derive(Subcommand)]
enum AssignmentCommand {
    Add {
        /// whom to assing
        #[arg(short, long)]
        person: i32,
        /// what chore to assign
        #[arg(short, long)]
        chore: i32,
    },
    /// command to remove the assignment
    Remove {
        /// index of the assignment to remove
        #[arg(short, long)]
        index: u8,
    },
    /// List assignments
    List,
}
#[derive(Subcommand)]
enum Commands {
    /// A command that manages persons in the household
    Person {
        /// Operations on a person
        #[command(subcommand)]
        command: PersonCommand,
    },
    /// A command that manages all chores
    Chore {
        /// Operations on a chore
        #[command(subcommand)]
        command: ChoreCommand,
    },
    /// Prints report for all persons, chores and assignments
    Report,
    /// Assigns a person to a chore
    Assignment {
        /// Operations on assignment
        #[command(subcommand)]
        command: AssignmentCommand,
    },
    Task {
        /// whom to assing
        #[arg(short, long)]
        person: i32,
        /// what chore to assign
        #[arg(short, long)]
        chore: i32,
        #[arg(short, long)]
        date: String,
    },
}

/// Parses the commandline and dispatches a command on [[data::Data]].
fn dispatch() -> Result<(), data::DataError> {
    let args = Cli::parse();
    let d: &mut dyn data::Data = &mut data::db(&PathBuf::from(args.dbpath));
    match &args.command {
        Commands::Person { command } => match command {
            PersonCommand::Add { name } => d.add_person(name),
            PersonCommand::Remove { index } => d.remove_person(*index),
            PersonCommand::List => d.list_persons(),
        },
        Commands::Chore { command } => match command {
            ChoreCommand::Add {
                description,
                level,
                freq_days,
            } => d.add_chore(description, *level, *freq_days),
            ChoreCommand::Remove { index } => d.remove_chore(*index),
            ChoreCommand::List => d.list_chores(),
        },
        Commands::Assignment { command } => match command {
            AssignmentCommand::Add { person, chore } => d.assign(*person, *chore),
            AssignmentCommand::Remove { index } => d.remove_assignment(*index),
            AssignmentCommand::List => d.list_assignments(),
        },
        Commands::Report => d.report(),
        Commands::Task {
            person,
            chore,
            date,
        } => d.add_task(*person, *chore, date),
    }
}
fn main() -> Result<(), data::DataError> {
    setup_panic!();
    dispatch()
}
