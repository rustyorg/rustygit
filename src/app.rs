use std::path::Path;

use anyhow::{anyhow, Result};
use git2::{Repository, StatusEntry, Statuses};
use tui::style::{Color, Style};
use tui::widgets::{ListItem, ListState};

use crate::git::{GetPath, StatusExt};

pub struct StatefulList<T> {
  pub state: ListState,
  pub items: T,
}

pub struct App<'a> {
  pub repo: &'a Repository,
  pub title: &'static str,
  pub list: StatefulList<Statuses<'a>>,
}

impl<'a> App<'a> {
  pub fn new(repo: &'a Repository) -> App<'a> {
    // This is just a placeholder example of getting a list of files from git.
    // See https://github.com/rust-lang/git2-rs/blob/master/examples/status.rs for
    // full examples of using the git status APIs.
    let statuses = repo.statuses(None).expect("Unable to get status.");

    App {
      repo,
      title: "RustyGit",
      list: StatefulList::with_items(statuses),
    }
  }

  pub fn refresh_statuses(&mut self) {
    let statuses = self.repo.statuses(None).expect("Unable to get status.");
    self.list.update_items(statuses);
  }

  pub fn primary_action(&mut self) -> Result<()> {
    let index = self.list.selected_index();
    if let Some(index) = index {
      let status = self.status_entry_at_index(index)?;
      let path = status.get_path()?;

      if status.status().is_staged() {
        self.reset_path(path)?;
      } else {
        self.add_path_to_index(path)?;
      }
    }
    Ok(())
  }

  /**
   * Resets that status of the given path to HEAD
   * i.e. any changes are removed from the index,
   * eqivilent to `git reset [path]`
   */
  fn reset_path(&self, path: &Path) -> Result<()> {
    let head = self.repo.head()?.peel_to_commit()?;
    self.repo.reset_default(Some(head.as_object()), [path])?;
    Ok(())
  }

  fn add_path_to_index(&self, path: &Path) -> Result<()> {
    let mut index = self.repo.index()?;
    index.add_path(path)?;
    index.write()?;
    Ok(())
  }

  fn status_entry_at_index(&self, index: usize) -> Result<StatusEntry> {
    self
      .list
      .items
      .get(index)
      .ok_or_else(|| anyhow!("Invalid status index"))
  }
}

pub trait DisplayList {
  fn len(&self) -> usize;
  fn list_items(&self) -> Vec<ListItem>;
}

impl<'a> DisplayList for Statuses<'a> {
  fn len(&self) -> usize {
    self.iter().count()
  }

  fn list_items(&self) -> Vec<ListItem> {
    self
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
      .collect()
  }
}

impl DisplayList for Vec<&str> {
  fn len(&self) -> usize {
    self.len()
  }

  fn list_items(&self) -> Vec<ListItem> {
    self
      .iter()
      .map(|i| ListItem::new(i.to_string()).style(Style::default()))
      .collect()
  }
}

impl<T: DisplayList> StatefulList<T> {
  pub fn with_items(items: T) -> Self {
    StatefulList {
      state: ListState::default(),
      items,
    }
  }

  fn update_items(&mut self, items: T) {
    self.items = items;
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

  pub fn selected_index(&self) -> Option<usize> {
    self.state.selected()
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
