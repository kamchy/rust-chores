use chrono::NaiveDate;
use rusqlite::params;
use rusqlite::Connection;
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::result::Result;
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
    fn get_persons(&self) -> Result<Vec<Person>, DataError>;
    fn remove_person(&mut self, index: u8) -> Result<(), DataError>;
    fn add_chore(
        self: &mut Self,
        description: &str,
        level: u8,
        freq_days: u8,
    ) -> Result<(), DataError>;
    fn remove_chore(&mut self, index: u8) -> Result<(), DataError>;

    fn get_chores(&self) -> Result<Vec<Chore>, DataError>;
    fn get_schedules(&self) -> Result<Vec<Schedule>, DataError>;
    /// id could be internal type
    fn assign(self: &mut Self, person_id: i32, chore_id: i32) -> Result<(), DataError>;
    fn remove_assignment(&mut self, index: u8) -> Result<(), DataError>;

    fn get_assignments(&self) -> Result<Vec<Assignment>, DataError>;
    fn add_task(&mut self, person_id: i32, chore_id: i32, date: &str) -> Result<(), DataError>;
}

pub struct RusqData {
    conn: Connection,
}

impl RusqData {
    fn new(path: &PathBuf) -> Result<Self, DataError> {
        let conn = Connection::open(path)?;
        conn.execute(fs::read_to_string("src/schema.sql")?.as_str(), ())?;
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
pub struct Assignment {
    id: i32,
    person_id: i32,
    chore_id: i32,
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
    fn add_task(&mut self, person_id: i32, chore_id: i32, date: &str) -> Result<(), DataError> {
        if let Ok(_) = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            self.conn
                .execute(
                    "insert into task(person_id, chore_id, done)  values(?1, ?2, ?3)",
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
    fn get_assignments(&self) -> Result<Vec<Assignment>, DataError> {
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
        let c: Vec<Assignment> = assignment_iter.filter_map(|r| r.ok()).collect();
        Ok(c)
    }

    fn get_schedules(&self) -> Result<Vec<Schedule>, DataError> {
        let mut stmt = self
            .conn
            .prepare(fs::read_to_string("src/report.sql")?.as_str())?;

        let iter = stmt.query_map([], |row| {
            let name = row.get(0)?;
            let description = row.get(1)?;
            let level = row.get(2)?;
            let frequency: u8 = row.get(3)?;
            let last: String = row.get(4)?;

            Ok(Schedule {
                name,
                description,
                level,
                frequency,
                last: last.clone(),
                next: calc_next_date(last, frequency),
            })
        })?;

        let mut schedules = Vec::new();
        for s in iter {
            schedules.push(s?);
        }
        Ok(schedules)
    }
}

fn calc_next_date(last: String, frequency: u8) -> String {
    NaiveDate::parse_from_str(last.as_str(), "%Y-%m-%d")
        .map(|d| d + chrono::Days::new(frequency.into()))
        .map(|nd| nd.to_string())
        .unwrap_or("unknown".to_string())
}

#[derive(Tabled)]
pub struct Schedule {
    name: String,
    description: String,
    level: u8,
    frequency: u8,
    last: String,
    next: String,
}

pub fn db(path: &PathBuf) -> RusqData {
    RusqData::new(path).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env::temp_dir;

    #[test]
    fn test_db_creation() -> Result<(), DataError> {
        let mut dbfile = temp_dir();
        dbfile.push("testdb");
        let _rd = RusqData::new(&dbfile);

        let conn = Connection::open(dbfile)?;

        // Query the sqlite_master table to check for the existence of tables
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
        let table_names: Vec<String> = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(|result| result.ok())
            .collect();

        for name in ["person", "chore", "assignment", "task"] {
            let expected = name.to_owned();
            assert_eq!(true, table_names.contains(&expected));
        }
        assert!(false);
        Ok(())
    }

    #[test]
    fn test_row_inserted() -> Result<(), DataError> {
        let mut dbfile = temp_dir();
        dbfile.push("testdb");
        let rd = RusqData::new(&dbfile);
        assert!(rd.is_ok());

        let res = rd.map(|mut r| r.add_person("anna")).ok();
        assert!(res.is_some());
        Ok(())
    }
}
