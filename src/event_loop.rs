use crossterm::event::{self, Event, KeyCode};
use std::{
  io,
  time::{Duration, Instant},
};
use tui::{backend::Backend, Terminal};

use crate::app::App;
use crate::ui::ui;

pub fn run_app<B: Backend>(
  terminal: &mut Terminal<B>,
  mut app: App,
  tick_rate: Duration,
) -> io::Result<()> {
  let mut last_tick = Instant::now();
  loop {
    terminal.draw(|f| ui(f, &mut app))?;

    let timeout = tick_rate
      .checked_sub(last_tick.elapsed())
      .unwrap_or_else(|| Duration::from_secs(0));
    if crossterm::event::poll(timeout)? {
      if let Event::Key(key) = event::read()? {
        match key.code {
          KeyCode::Char('q') => return Ok(()),
          KeyCode::Left => app.list.unselect(),
          KeyCode::Down => app.list.next(),
          KeyCode::Up => app.list.previous(),
          KeyCode::Char(' ') => {
            if let Err(_error) = app.primary_action() {
              // TODO, display errors in a panel
            };
          }
          _ => (),
        };
      }
    }
    if last_tick.elapsed() >= tick_rate {
      last_tick = Instant::now();
    }
  }
}
