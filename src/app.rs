use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<String>,
}

pub struct App {
    pub title: String,
    pub items: StatefulList,
}

impl App {
    pub fn new() -> App {
        App {
            title: "RustyGit".to_string(),
            items: StatefulList::with_items(vec![
                "Item0".to_string(),
            ])
        }
    }
  }

impl StatefulList {
    pub fn with_items(items: Vec<String>) -> Self {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
