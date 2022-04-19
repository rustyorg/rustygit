use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::Paragraph;
use tui::{
  backend::Backend,
  layout::{Constraint, Direction, Layout},
  style::{Modifier, Style},
  widgets::{List, ListItem},
  Frame,
};

use crate::app::{App, DisplayList};
use crate::ui::helpers::create_block;

mod helpers;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  // Create two chunks with equal horizontal screen space
  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    .split(f.size());

  render_left_view(f, app, chunks[0]);
}

pub fn render_left_view<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(3), Constraint::Percentage(90)].as_ref())
    .split(area);

  render_status(f, app, chunks[0]);
  render_files(f, app, chunks[1])
}

pub fn render_status<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
  let (ahead, behind) = app.branch.ahead_behind.unwrap();
  // let count = app.branch.branch_local.get().is_branch();
  let name = format!("{}↑ {}↓ -> {:?}", ahead, behind, app.branch.name.clone());
  let paragraph = Paragraph::new(name)
    .style(Style::default().fg(Color::White))
    .block(create_block("Status"));
  f.render_widget(paragraph, area)
}

pub fn render_files<B: Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
  // Iterate through all elements in the `items` app and append some debug text to it.
  let items: Vec<ListItem> = app.list.items.list_items();

  // Create a List from all list items and highlight the currently selected one
  let items = List::new(items)
    .block(create_block(app.title))
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol(">> ");

  // We can now render the item list
  f.render_stateful_widget(items, area, &mut app.list.state);
}
