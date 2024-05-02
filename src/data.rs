use chrono::Duration;
use rusqlite::Connection;
use std::path::PathBuf;
use std::result::Result;
use tabled::settings::{Color, Style};
use tabled::Tabled;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("Could not insert")]
    InsertError(#[from] rusqlite::Error),
}
pub trait Data {
    fn add_person(&mut self, name: &str) -> Result<(), DataError>;
    fn add_chore(
        self: &mut Self,
        description: &str,
        level: u8,
        freq_days: u8,
    ) -> Result<(), DataError>;
    fn report(self: &Self) -> Result<(), DataError>;
    /// id could be internal type
    fn assign(self: &mut Self, person_id: i32, chore_id: i32) -> Result<(), DataError>;
}

pub struct RusqData {
    conn: Connection,
}
impl RusqData {
    fn new(path: &PathBuf) -> Result<Self, DataError> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE  IF NOT EXISTS person(
                id INTEGER PRIMARY  KEY AUTOINCREMENT,
                name TEXT NOT NULL
            ) ",
            (),
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS chore (
                id INTEGER PRIMARY  KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                level INTEGER NOT NULL,
                frequency INTEGER NOT NULL
            ) ",
            (),
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS assignment(
                id INTEGER PRIMARY  KEY AUTOINCREMENT,
                person_id INTEGER NOT NULL REFERENCES person(id) ON DELETE CASCADE,
               chore_id INTEGER NOT NULL REFERENCES chore(id) ON DELETE CASCADE
            ) ",
            (),
        )?;

        Ok(RusqData { conn })
    }
}

#[derive(Debug, Tabled)]
struct Person {
    id: i32,
    name: String,
}

#[derive(Debug, Tabled)]
struct Chore {
    id: i32,
    description: String,
    level: u8,
    frequency: u8,
}

#[derive(Debug, Tabled)]
struct Assignment {
    id: i32,
    person_id: i32,
    chore_id: i32,
}
fn date_from(d: chrono::DateTime<chrono::Local>, after_days: u8) -> String {
    let days = Duration::days(after_days as i64);
    let next_date = d + days;
    let formatted = next_date.format("%Y-%m-%d");
    formatted.to_string()
}

/// Implements Data so that it is stored in sqlite database
/// using rusql driver library

impl Data for RusqData {
    fn add_person(self: &mut Self, n: &str) -> Result<(), DataError> {
        let p = Person {
            id: 0,
            name: n.to_string(),
        };
        self.conn
            .execute("INSERT INTO person (name) VALUES (?1)", [p.name])
            .map_err(|e| DataError::InsertError(e))
            .map(|_| {})
    }

    fn add_chore(
        self: &mut Self,
        description: &str,
        level: u8,
        freq_days: u8,
    ) -> Result<(), DataError> {
        let c = Chore {
            id: 0,
            level,
            frequency: freq_days,
            description: description.to_string(),
        };
        self.conn
            .execute(
                " INSERT INTO chore(description, level, frequency) VALUES (?1, ?2, ?3)",
                (c.description, c.level, c.frequency),
            )
            .map_err(|r| DataError::InsertError(r))
            .map(|_| {})
    }

    fn report(self: &Self) -> Result<(), DataError> {
        print_all(&self.conn)
    }

    fn assign(self: &mut Self, person_id: i32, chore_id: i32) -> Result<(), DataError> {
        let assignment = Assignment {
            id: 0,
            person_id,
            chore_id,
        };
        self.conn
            .execute(
                "INSERT INTO assignment (person_id, chore_id) VALUES(?1, ?2)",
                [assignment.person_id, assignment.chore_id],
            )
            .map_err(|e| DataError::InsertError(e))
            .map(|_| {})
    }
}

fn print_all(conn: &Connection) -> Result<(), DataError> {
    let mut person_stmt = conn.prepare("SELECT id, name FROM person")?;
    let person_iter = person_stmt.query_map([], |row| {
        let id = row.get(0)?;
        let name: String = row.get(1)?;
        Ok(Person { id, name })
    })?;

    let mut chore_stmnt = conn.prepare("SELECT id, description, level, frequency FROM chore")?;
    let chore_iter = chore_stmnt.query_map([], |row| {
        let id = row.get(0)?;
        let description = row.get(1)?;
        let level = row.get(2)?;
        let frequency = row.get(3)?;
        Ok(Chore {
            id,
            description,
            level,
            frequency,
        })
    })?;
    let mut assignment_stmnt = conn.prepare("SELECT id, person_id, chore_id FROM assignment")?;
    let assignment_iter = assignment_stmnt.query_map([], |row| {
        let id = row.get(0)?;
        let person_id = row.get(1)?;
        let chore_id = row.get(2)?;
        Ok(Assignment {
            id,
            person_id,
            chore_id,
        })
    })?;

    let p: Vec<Person> = person_iter.filter_map(|r| r.ok()).collect();
    println!(
        "{}\n{}",
        p.len(),
        tabled::Table::new(p).with(Style::rounded()).to_string()
    );
    let c: Vec<Chore> = chore_iter.filter_map(|r| r.ok()).collect();
    println!(
        "{}",
        tabled::Table::new(c).with(Style::rounded()).to_string()
    );

    let a: Vec<Assignment> = assignment_iter.filter_map(|r| r.ok()).collect();
    println!(
        "{}",
        tabled::Table::new(a).with(Style::rounded()).to_string()
    );
    Ok(())
}

pub fn db(path: &PathBuf) -> RusqData {
    RusqData::new(path).unwrap()
}
