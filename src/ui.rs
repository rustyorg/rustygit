use tui::{
  backend::Backend,
  layout::{Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  widgets::{Block, Borders, List, ListItem},
  Frame,
};

use crate::app::App;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  // Create two chunks with equal horizontal screen space
  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    .split(f.size());

  // Iterate through all elements in the `items` app and append some debug text to it.
  let items: Vec<ListItem> = app
    .items
    .items
    .iter()
    .map(|i| {
      let colour = if i.status().is_wt_new() {
        Color::Yellow
      } else if i.status().is_wt_modified() {
        Color::Green
      } else if i.status().is_wt_deleted() {
        Color::Red
      } else {
        Color::White
      };

      ListItem::new(i.path().expect("no path defined!").to_string())
        .style(Style::default().fg(colour))
    })
    .collect();

  // Create a List from all list items and highlight the currently selected one
  let items = List::new(items)
    .block(
      Block::default()
        .borders(Borders::ALL)
        .title(app.title.as_str()),
    )
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    .highlight_symbol(">> ");

  // We can now render the item list
  f.render_stateful_widget(items, chunks[0], &mut app.items.state);
}
