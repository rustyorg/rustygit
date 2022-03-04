use git2::Repository;
use tui::widgets::ListState;

use crate::git;

pub struct StatefulList<T> {
  pub state: ListState,
  pub items: Vec<T>,
}

pub struct App {
  pub repo: Repository,
  pub title: String,
  pub items: StatefulList<String>,
}

impl App {
  pub fn new() -> App {
    let repo = git::open_current_repo();
    // This is just a placeholder example of getting a list of files from git.
    // See https://github.com/rust-lang/git2-rs/blob/master/examples/status.rs for
    // full examples of using the git status APIs.
    let filenames = repo
      .statuses(None)
      .expect("Unable to get status.")
      .iter()
      .filter_map(|s| s.path().map(|p| p.to_string()))
      .collect();

    App {
      repo,
      title: "RustyGit".to_string(),
      items: StatefulList::with_items(filenames),
    }
  }
}

impl<T> StatefulList<T> {
  pub fn with_items(items: Vec<T>) -> Self {
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
