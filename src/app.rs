use std::marker::PhantomData;

use git2::{build::RepoBuilder, Repository, StatusEntry, StatusOptions, Statuses};
use tui::{
  style::{Color, Style},
  widgets::{ListItem, ListState},
};

use crate::git;

// struct DisplayItem {
//   colour: Color,
//   text: String,
//   index: usize
// }

pub trait DisplayList {
  fn display_list(&self) -> Vec<ListItem>;

  fn len(&self) -> usize {
    self.display_list().len()
  }
}

// pub struct StatefulList<T>
// {
//   pub state: ListState,
//   pub items: Vec<T>,
// }
pub struct StatefulList<T> {
  pub state: ListState,
  pub items: T,
}

pub struct App<'a> {
  pub repo: &'a Repository,
  pub title: String,
  pub items: StatefulList<Statuses<'a>>,
}

impl<'a> DisplayList for Statuses<'a> {
  fn display_list(&self) -> Vec<ListItem> {
    self.iter().map(status_entry_to_list_item).collect()
  }
}

fn status_entry_to_list_item(s: StatusEntry) -> ListItem {
  let filename = s.path().unwrap_or("").to_string();
  let status = s.status();
  let colour = if status.is_wt_modified() {
    Color::Red
  } else if status.is_wt_new() {
    Color::LightBlue
  } else {
    Color::Gray
  };
  ListItem::new(filename).style(Style::default().fg(colour).bg(Color::Black))
}

impl DisplayList for Vec<&str> {
  fn display_list(&self) -> Vec<ListItem> {
    self
      .iter()
      .map(|i| ListItem::new(*i).style(Style::default().fg(Color::Black).bg(Color::White)))
      .collect()
  }
}

impl<'a> App<'a> {
  pub fn new(repo: &'a Repository) -> Self {
    // This is just a placeholder example of getting a list of files from git.
    // See https://github.com/rust-lang/git2-rs/blob/master/examples/status.rs for
    // full examples of using the git status APIs.
    let mut status_opts = StatusOptions::new();
    status_opts.include_ignored(false);
    status_opts.include_untracked(true);

    let statuses: Statuses<'a> = repo
      .statuses(Some(&mut status_opts))
      .expect("error getting git status");

    App {
      repo,
      title: "RustyGit".to_string(),
      items: StatefulList::with_items(statuses),
    }
  }
}

impl<T> StatefulList<T>
where
  T: DisplayList,
{
  pub fn with_items(items: T) -> Self {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_starts_at_none() {
    let list = StatefulList::with_items(vec!["a", "b", "c"]);

    assert_eq!(list.state.selected(), None);
  }

  #[test]
  fn test_next_selects() {
    let mut list = StatefulList::with_items(vec!["a", "b", "c"]);

    list.next();

    assert_eq!(list.state.selected(), Some(0));
  }

  #[test]
  fn test_next_increments() {
    let mut list = StatefulList::with_items(vec!["a", "b", "c"]);

    list.next();
    list.next();

    assert_eq!(list.state.selected(), Some(1));
  }

  #[test]
  fn test_next_wrap() {
    let mut list = StatefulList::with_items(vec!["a", "b"]);

    list.next();
    list.next();

    assert_eq!(list.state.selected(), Some(1));

    list.next();
    assert_eq!(list.state.selected(), Some(0));
  }

  #[test]
  fn test_previous_selects() {
    let mut list = StatefulList::with_items(vec!["a", "b", "c"]);

    list.previous();

    assert_eq!(list.state.selected(), Some(0));
  }

  #[test]
  fn test_previous_decrements() {
    let mut list = StatefulList::with_items(vec!["a", "b", "c"]);

    list.previous();
    list.previous();

    assert_eq!(list.state.selected(), Some(2));
  }

  #[test]
  fn test_unselect() {
    let mut list = StatefulList::with_items(vec!["a", "b", "c"]);

    list.next();

    assert_eq!(list.state.selected(), Some(0));

    list.unselect();

    assert_eq!(list.state.selected(), None);
  }
}
