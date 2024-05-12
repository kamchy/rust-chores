use tabled::settings::Style;
use tabled::Tabled;

pub trait Renderer<T: Tabled> {
    fn render_table(&self, v: Vec<T>) -> String;
}

pub struct TabledRenderer;

impl<T: Tabled> Renderer<T> for TabledRenderer {
    fn render_table(&self, v: Vec<T>) -> String {
        tabled::Table::new(v).with(Style::rounded()).to_string()
    }
}
