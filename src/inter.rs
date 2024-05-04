use std::path::PathBuf;

use anyhow::anyhow;
use data::Data;
use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};
use inquire::{Confirm, DateSelect, Select};
mod data;
fn main() -> Result<(), anyhow::Error> {
    let mut d = data::db(&PathBuf::from("test.db"));
    inquire::set_global_render_config(get_render_config());
    let _date = DateSelect::new("Date:").prompt()?;
    let _person = Select::new("Person:", get_persons(&d)).prompt()?;
    let _chore = Select::new("Chore:", get_chores(&d)).prompt()?;

    Confirm::new(
        format!(
            "You will save that {} did {} at {}. Is that correct?",
            _person, _chore, _date
        )
        .as_str(),
    )
    .with_default(true)
    .prompt()
    .map(|_| {
        d.add_task(_person.id, _chore.id, _date.to_string().as_str())
            .unwrap()
    })
    .map_err(|e| anyhow!(e))
}
fn get_chores(d: &impl data::Data) -> Vec<data::Chore> {
    d.get_chores().ok().unwrap_or(vec![])
}
fn get_persons(d: &impl data::Data) -> Vec<data::Person> {
    d.get_persons()
        .unwrap_or(vec![])
        .into_iter()
        // .map(|p| p.name().to_owned())
        .collect()
}

fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new("$").with_fg(Color::LightRed);
    render_config.highlighted_option_prefix = Styled::new("➠").with_fg(Color::LightYellow);
    render_config.selected_checkbox = Styled::new("☑").with_fg(Color::LightGreen);
    render_config.scroll_up_prefix = Styled::new("⇞");
    render_config.scroll_down_prefix = Styled::new("⇟");
    render_config.unselected_checkbox = Styled::new("☐");

    render_config.error_message = render_config
        .error_message
        .with_prefix(Styled::new("❌").with_fg(Color::LightRed));

    render_config.answer = StyleSheet::new()
        .with_attr(Attributes::ITALIC)
        .with_fg(Color::LightYellow);

    render_config.help_message = StyleSheet::new().with_fg(Color::DarkYellow);

    render_config
}
