use chrono::Duration;
use rusqlite::params;
use rusqlite::Connection;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::result::Result;
use tabled::settings::Style;
use tabled::Tabled;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("Could not insert")]
    InsertError(#[from] rusqlite::Error),
    #[error("Cound not read query from file {0:}")]
    QuryFilePerseError(#[from] std::io::Error),

    #[error("Could not delete {table:?} with id {index:?}: \n{message:?}")]
    DeleteError {
        table: String,
        index: u8,
        message: String,
    },
    #[error("Error when parsing data to be stored")]
    ParseError(String),
}
pub trait Data {
    fn add_person(&mut self, name: &str) -> Result<(), DataError>;
    fn list_persons(&self) -> Result<(), DataError>;
    fn get_persons(&self) -> Result<Vec<Person>, DataError>;
    fn remove_person(&mut self, index: u8) -> Result<(), DataError>;
    fn add_chore(
        self: &mut Self,
        description: &str,
        level: u8,
        freq_days: u8,
    ) -> Result<(), DataError>;
    fn remove_chore(&mut self, index: u8) -> Result<(), DataError>;
    fn list_chores(&self) -> Result<(), DataError>;

    fn get_chores(&self) -> Result<Vec<Chore>, DataError>;
    fn report(self: &Self) -> Result<(), DataError>;
    /// id could be internal type
    fn assign(self: &mut Self, person_id: i32, chore_id: i32) -> Result<(), DataError>;
    fn list_assignments(&self) -> Result<(), DataError>;
    fn remove_assignment(&mut self, index: u8) -> Result<(), DataError>;
    fn add_task(&mut self, person_id: i32, chore_id: i32, date: &str) -> Result<(), DataError>;
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
                name TEXT UNIQUE NOT NULL
            ) ",
            (),
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS chore (
                id INTEGER PRIMARY  KEY AUTOINCREMENT,
                description TEXT UNIQUE NOT NULL,
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

        conn.execute(
            "CREATE TABLE IF NOT EXISTS task(
                id INTEGER PRIMARY  KEY AUTOINCREMENT,
                person_id INTEGER NOT NULL REFERENCES person(id) ON DELETE CASCADE,
                chore_id INTEGER NOT NULL REFERENCES chore(id) ON DELETE CASCADE,
                done DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            ) ",
            (),
        )?;
        Ok(RusqData { conn })
    }
}

#[derive(Debug, Tabled)]
pub struct Person {
    pub(crate) id: i32,
    pub(crate) name: String,
}
impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} (id:{:0>4})", self.name, self.id))
    }
}

#[derive(Debug, Tabled)]
pub struct Chore {
    pub(crate) id: i32,
    pub(crate) description: String,
    level: u8,
    pub(crate) frequency: u8,
}

impl Display for Chore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} (id:{:0>4}) [fr: {}]",
            self.description, self.id, self.frequency
        ))
    }
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
    fn list_persons(&self) -> Result<(), DataError> {
        print_vec(&self.conn, get_persons)
    }

    fn get_persons(&self) -> Result<Vec<Person>, DataError> {
        let mut person_stmt = self.conn.prepare("SELECT id, name FROM person")?;
        let person_iter = person_stmt.query_map([], |row| {
            let id = row.get(0)?;
            let name: String = row.get(1)?;
            Ok(Person { id, name })
        })?;

        let p: Vec<Person> = person_iter.filter_map(|r| r.ok()).collect();
        Ok(p)
    }
    fn remove_person(self: &mut Self, index: u8) -> Result<(), DataError> {
        self.conn
            .execute("DELETE FROM person where id = ?1", [index])
            .map_err(|e| DataError::DeleteError {
                table: "person".to_owned(),
                index,
                message: e.to_string(),
            })
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

    fn remove_chore(self: &mut Self, index: u8) -> Result<(), DataError> {
        self.conn
            .execute("DELETE FROM chore where id = ?1", [index])
            .map_err(|e| DataError::DeleteError {
                table: "chore".to_owned(),
                index,
                message: e.to_string(),
            })
            .map(|_| {})
    }

    fn list_chores(&self) -> Result<(), DataError> {
        print_vec(&self.conn, get_chores)
    }

    fn get_chores(&self) -> Result<Vec<Chore>, DataError> {
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

        let c: Vec<Chore> = chore_iter.filter_map(|r| r.ok()).collect();
        Ok(c)
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

    fn remove_assignment(self: &mut Self, index: u8) -> Result<(), DataError> {
        self.conn
            .execute("DELETE FROM assignment where id = ?1", [index])
            .map_err(|e| DataError::DeleteError {
                table: "assignment".to_owned(),
                index,
                message: e.to_string(),
            })
            .map(|_| {})
    }
    fn list_assignments(&self) -> Result<(), DataError> {
        print_vec(&self.conn, get_assignments)
    }
    fn add_task(&mut self, person_id: i32, chore_id: i32, date: &str) -> Result<(), DataError> {
        if let Ok(_) = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            self.conn
                .execute(
                    "insert into task(person_id, chore_id, done) 
            values(?1, ?2, ?3)",
                    params![person_id, chore_id, date],
                )
                .map_err(|e| DataError::InsertError(e))
                .map(|_| {})
        } else {
            Err(DataError::ParseError(format!(
                "Could not parse date {:?}, should match format %Y-%M-%D",
                date
            )))
        }
    }
}
#[derive(Tabled)]
struct Schedule {
    name: String,
    description: String,
    level: u8,
    frequency: u8,
    last: String,
}
fn print_all(conn: &Connection) -> Result<(), DataError> {
    let mut stmt = conn.prepare(fs::read_to_string("src/report.sql")?.as_str())?;

    let iter = stmt.query_map([], |row| {
        let name = row.get(0)?;
        let description = row.get(1)?;
        let level = row.get(2)?;
        let frequency = row.get(3)?;
        let last = row.get(4)?;

        Ok(Schedule {
            name,
            description,
            level,
            frequency,
            last,
        })
    })?;
    let report: Vec<_> = iter.filter_map(|r| r.ok()).collect();
    println!(
        "{}",
        tabled::Table::new(report)
            .with(Style::rounded())
            .to_string()
    );
    Ok(())
}
// remove, use method in list_persons
fn get_persons(conn: &Connection) -> Result<Vec<Person>, DataError> {
    let mut person_stmt = conn.prepare("SELECT id, name FROM person")?;
    let person_iter = person_stmt.query_map([], |row| {
        let id = row.get(0)?;
        let name: String = row.get(1)?;
        Ok(Person { id, name })
    })?;

    let p: Vec<Person> = person_iter.filter_map(|r| r.ok()).collect();
    Ok(p)
}

fn get_chores(conn: &Connection) -> Result<Vec<Chore>, DataError> {
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

    let c: Vec<Chore> = chore_iter.filter_map(|r| r.ok()).collect();
    Ok(c)
}

fn get_assignments(conn: &Connection) -> Result<Vec<Assignment>, DataError> {
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
    let a: Vec<Assignment> = assignment_iter.filter_map(|r| r.ok()).collect();
    Ok(a)
}
fn print_vec<'a, T: Tabled>(
    conn: &'a Connection,
    f: fn(&'a Connection) -> Result<Vec<T>, DataError>,
) -> Result<(), DataError> {
    let v = f(conn)?;
    println!(
        "{}",
        tabled::Table::new(v).with(Style::rounded()).to_string()
    );
    Ok(())
}

pub fn db(path: &PathBuf) -> RusqData {
    RusqData::new(path).unwrap()
}
