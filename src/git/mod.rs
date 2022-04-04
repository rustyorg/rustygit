/**
 * This module contains helper methods for types in the git2 library
 * in order to provide a higher-level or simpler API where useful.
 */
use std::path::Path;

use anyhow::{anyhow, Result};
use git2::{Repository, Status, StatusEntry};

pub fn open_current_repo() -> Repository {
  match Repository::open(".") {
    Ok(repo) => repo,
    Err(e) => panic!("failed to open: {}", e),
  }
}

pub trait StatusExt {
  fn is_staged(&self) -> bool;
}

impl StatusExt for Status {
  fn is_staged(&self) -> bool {
    self.is_index_new()
      || self.is_index_modified()
      || self.is_index_deleted()
      || self.is_index_renamed()
      || self.is_index_typechange()
  }
}

pub trait GetPath {
  fn get_path(&self) -> Result<&Path>;
}

impl GetPath for StatusEntry<'_> {
  fn get_path(&self) -> Result<&Path> {
    let path_str = self.path().ok_or_else(|| anyhow!("Invalid path"))?;
    Ok(Path::new(path_str))
  }
}
