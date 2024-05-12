use tabled::settings::Style;
use tabled::Tabled;

use crate::data::{Assignment, Chore, Person, Schedule};

pub trait Renderer<T: Tabled> {
    fn render_table(&self, v: Vec<T>) -> String;
}

pub struct TabledRenderer;

impl<T: Tabled> Renderer<T> for TabledRenderer {
    fn render_table(&self, v: Vec<T>) -> String {
        tabled::Table::new(v).with(Style::rounded()).to_string()
    }
}

impl Tabled for Person {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        vec![
            std::borrow::Cow::Owned(self.id.to_string()),
            std::borrow::Cow::Borrowed(&self.name),
        ]
    }
    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["Id".into(), "Name".into()]
    }
}

impl Tabled for Schedule {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        vec![
            std::borrow::Cow::Borrowed(&self.name),
            std::borrow::Cow::Borrowed(&self.description),
            std::borrow::Cow::Owned(self.level.to_string()),
            std::borrow::Cow::Owned(self.frequency.to_string()),
            std::borrow::Cow::Borrowed(&self.last),
            std::borrow::Cow::Borrowed(&self.next),
        ]
    }
    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        ["Name", "Desc", "Level", "Freq", "Last", "Next"]
            .iter()
            .map(|s| std::borrow::Cow::Borrowed(*s))
            .collect()
    }
}

impl Tabled for Assignment {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        vec![
            std::borrow::Cow::Owned(self.id.to_string()),
            std::borrow::Cow::Owned(self.person_id.to_string()),
            std::borrow::Cow::Owned(self.chore_id.to_string()),
        ]
    }
    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "person_id".into(), "chore_id".into()]
    }
}
impl Tabled for Chore {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'_, str>> {
        vec![
            std::borrow::Cow::Owned(self.id.to_string()),
            std::borrow::Cow::Borrowed(&self.description),
            std::borrow::Cow::Owned(self.level.to_string()),
            std::borrow::Cow::Owned(self.frequency.to_string()),
        ]
    }
    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "description".into(),
            "level".into(),
            "frequency".into(),
        ]
    }
}
