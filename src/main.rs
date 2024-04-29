use chrono::Duration;
use clap::{Parser, Subcommand};
use human_panic::setup_panic;
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
        person: String,
        /// what chore to assign
        #[arg(short, long)]
        chore: String,
    },
}

fn add_person(name: &str) {
    println!("Adding person {name:?}")
}

fn add_chore(description: &str, level: u8, freq_days: Duration) {
    let next_date = chrono::Local::now() + freq_days;
    let formatted = next_date.format("%Y-%m-%d");
    println!("Adding chore {description:?} with level {level:?} - days left: {freq_days:?} which means {formatted:?} ")
}

fn report() {
    println!("reporting")
}

fn assign(person: &str, chore: &str) {
    println!("Assining {person} to {chore}")
}
fn main() {
    setup_panic!();
    let args = Cli::parse();
    match &args.command {
        Commands::AddPerson { name } => add_person(name),
        Commands::AddChore {
            description,
            level,
            freq_days,
        } => add_chore(description, *level, Duration::days(*freq_days as i64)),
        Commands::Report => report(),
        Commands::Assign { person, chore } => assign(person, chore),
    }
}
