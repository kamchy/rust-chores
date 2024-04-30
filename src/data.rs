use std::path::PathBuf;

use chrono::Duration;
use rusqlite::{Connection, Result};
pub trait Data {
    fn add_person(self: &mut Self, name: &str) -> Result<usize>;
    fn add_chore(self: &mut Self, description: &str, level: u8, freq_days: u8) -> Result<usize>;
    fn report(self: &Self) -> Result<()>;
    /// id could be internal type
    fn assign(self: &mut Self, person_id: i32, chore_id: i32) -> Result<usize>;
}

pub struct RusqData {
    conn: Connection,
}
impl RusqData {
    fn new(path: &PathBuf) -> Result<Self> {
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

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
}

#[derive(Debug)]
struct Chore {
    id: i32,
    description: String,
    level: u8,
    frequency: u8,
}

#[derive(Debug)]
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
/// EchoData only echoes data back and does not store it
pub struct EchoData;

impl Data for EchoData {
    fn add_person(self: &mut Self, name: &str) -> Result<usize> {
        println!("Adding person {name:?}");
        Ok(1)
    }

    fn add_chore(self: &mut Self, description: &str, level: u8, freq_days: u8) -> Result<usize> {
        let formatted = date_from(chrono::Local::now(), freq_days);
        println!("Adding chore {description:?} with level {level:?} - days left: {freq_days:?} which means {formatted:?} ");
        Ok(1)
    }

    fn report(self: &Self) -> Result<()> {
        println!("reporting");
        Ok(())
    }

    fn assign(self: &mut Self, person: i32, chore: i32) -> Result<usize> {
        println!("Assining {person} to {chore}");
        Ok(1)
    }
}
/// Implements Data so that it is stored in sqlite database
/// using rusql driver library

impl Data for RusqData {
    fn add_person(self: &mut Self, n: &str) -> Result<usize> {
        let p = Person {
            id: 0,
            name: n.to_string(),
        };
        self.conn
            .execute("INSERT INTO person (name) VALUES (?1)", [p.name])
    }

    fn add_chore(self: &mut Self, description: &str, level: u8, freq_days: u8) -> Result<usize> {
        let c = Chore {
            id: 0,
            level,
            frequency: freq_days,
            description: description.to_string(),
        };
        self.conn.execute(
            " INSERT INTO chore(description, level, frequency) VALUES (?1, ?2, ?3)",
            (c.description, c.level, c.frequency),
        )
    }

    fn report(self: &Self) -> Result<()> {
        let mut person_stmt = self.conn.prepare("SELECT id, name FROM person")?;
        let person_iter = person_stmt.query_map([], |row| {
            let id = row.get(0)?;
            let name: String = row.get(1)?;
            Ok(Person { id, name })
        })?;

        let mut chore_stmnt = self
            .conn
            .prepare("SELECT id, description, level, frequency FROM chore")?;
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
        let mut assignment_stmnt = self
            .conn
            .prepare("SELECT id, person_id, chore_id FROM assignment")?;

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

        for person in person_iter {
            println!("{:?}", person)
        }

        for chore in chore_iter {
            println!("{:?}", chore)
        }

        for assignment in assignment_iter {
            println!("{:?}", assignment)
        }

        Ok(())
    }

    fn assign(self: &mut Self, person_id: i32, chore_id: i32) -> Result<usize> {
        let assignment = Assignment {
            id: 0,
            person_id,
            chore_id,
        };
        self.conn.execute(
            "INSERT INTO assignment (person_id, chore_id) VALUES(?1, ?2)",
            [assignment.person_id, assignment.chore_id],
        )?;
        Ok(0)
    }
}
pub fn db(path: &PathBuf) -> RusqData {
    RusqData::new(path).unwrap()
}
