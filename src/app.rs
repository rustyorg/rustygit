use git2::{Repository, Statuses};
use tui::widgets::ListState;

pub struct StatefulList<T> {
  pub state: ListState,
  pub items: T,
}

pub struct App<'a> {
  pub repo: &'a Repository,
  pub title: String,
  pub items: StatefulList<Statuses<'a>>,
}

impl<'a> App<'a> {
  pub fn new(repo: &'a Repository) -> App<'a> {
    // This is just a placeholder example of getting a list of files from git.
    // See https://github.com/rust-lang/git2-rs/blob/master/examples/status.rs for
    // full examples of using the git status APIs.
    let statuses = repo.statuses(None).expect("Unable to get status.");

    App {
      repo,
      title: "RustyGit".to_string(),
      items: StatefulList::with_items(statuses),
    }
  }
}

trait DisplayList {
  fn len(&self) -> usize;
}

impl<'a> DisplayList for Statuses<'a> {
  fn len(&self) -> usize {
    self.iter().count()
  }
}

impl<T> DisplayList for Vec<T> {
  fn len(&self) -> usize {
    self.len()
  }
}

impl<T: DisplayList> StatefulList<T> {
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
