/**
 * This module contains helper methods for types in the git2 library
 * in order to provide a higher-level or simpler API where useful.
 */
use std::path::Path;

use anyhow::{anyhow, Result};
use git2::{BranchType, Error, ErrorCode, ObjectType, Repository, Status, StatusEntry};

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

pub struct Branch<'a> {
  pub name: String,
  pub branch_remote: Option<git2::Branch<'a>>,
  pub branch_local: git2::Branch<'a>,
  pub ahead_behind: Option<(usize, usize)>,
}

pub fn get_current_branch_name(repo: &Repository) -> Result<String, Error> {
  let head = match repo.head() {
    Ok(head) => Some(head),
    Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => None,
    Err(e) => return Err(e),
  };

  match head.as_ref().and_then(|h| h.shorthand()) {
    None => Ok("".to_string()),
    Some(branch) => Ok(branch.to_string()),
  }
}

pub fn get_current_branch(repo: &Repository) -> Result<Branch, Error> {
  let head = match repo.head() {
    Ok(head) => Some(head),
    Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => None,
    Err(_e) => None,
  };

  let name = head.as_ref().and_then(|h| h.shorthand()).unwrap_or("-");
  let name_remote = format!("origin/{}", name);

  let branch_local = repo.find_branch(name, BranchType::Local).unwrap();
  let branch_remote = match repo.find_branch(name_remote.as_str(), BranchType::Remote) {
    Ok(branch) => Some(branch),
    Err(ref e) if e.code() == ErrorCode::UnbornBranch || e.code() == ErrorCode::NotFound => None,
    Err(e) => return Err(e),
  };

  let mut ahead_behind = Some((0, 0));
  let local_oid = branch_local.get().peel(ObjectType::Commit).unwrap().id();

  if branch_remote.is_some() {
    let remote_oid = branch_remote
      .as_ref()
      .unwrap()
      .get()
      .peel(ObjectType::Commit)
      .unwrap()
      .id();

    ahead_behind = match repo.graph_ahead_behind(local_oid, remote_oid) {
      Ok(values) => Some(values),
      Err(_) => Some((0, 0)),
    }
  };

  Ok(Branch {
    name: name.to_string(),
    branch_remote,
    branch_local,
    ahead_behind,
  })
}
