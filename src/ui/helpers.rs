use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders};

// Global styling block
pub fn create_block(title: &'static str) -> Block {
  Block::default()
    .borders(Borders::ALL)
    .style(Style::default().fg(Color::DarkGray))
    .title(Span::styled(title, Style::default().fg(Color::Gray)))
}
