use crate::app::{App, DisplayList};
use tui::{
  backend::Backend,
  layout::{Constraint, Direction, Layout},
  style::{Modifier, Style},
  widgets::{Block, Borders, List, ListItem},
  Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  // Create two chunks with equal horizontal screen space
  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
    .split(f.size());

  // Iterate through all elements in the `items` app and append some debug text to it.
  let items: Vec<ListItem> = app.list.items.list_items();

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
  f.render_stateful_widget(items, chunks[0], &mut app.list.state);
}
