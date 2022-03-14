use std::marker::PhantomData;

use git2::{build::RepoBuilder, Repository, StatusEntry, StatusOptions, Statuses};
use tui::widgets::ListState;

use crate::git;

pub trait DisplayList {
  fn display_list(&self) -> Vec<String>;

  fn len(&self) -> usize {
    self.display_list().len()
  }
}

pub struct StatefulList<T>
where
  T: DisplayList,
{
  pub state: ListState,
  pub items: T,
}

pub struct App<'a> {
  pub repo: &'a Repository,
  pub title: String,
  pub items: StatefulList<Statuses<'a>>,
}

impl<'a> DisplayList for Statuses<'a> {
  fn display_list(&self) -> Vec<String> {
    self.iter().filter_map(|ref s | s.path().map(|p| p.to_string())).collect()
  }
}

impl DisplayList for Vec<&str> {
    fn display_list(&self) -> Vec<String> {
      self.iter().map(|i| i.to_string()).collect()
    }
}

impl<'a> App<'a> {
  pub fn new(repo: &'a Repository) -> Self {
    // This is just a placeholder example of getting a list of files from git.
    // See https://github.com/rust-lang/git2-rs/blob/master/examples/status.rs for
    // full examples of using the git status APIs.
    let mut status_opts = StatusOptions::new();
    status_opts.include_ignored(false);

    let statuses = repo.statuses(Some(&mut status_opts)).expect("can't do it");

    App {
      repo,
      title: "RustyGit".to_string(),
      items: StatefulList::with_items(statuses),
    }
  }
}

impl<T: DisplayList> StatefulList<T>
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

  pub fn previous(& mut self) {
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
